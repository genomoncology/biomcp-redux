//! Synthetic protobuf fixtures prove the bounded tensor codec through its public surface.
use biomcp_fetch::grpc::{CodecError, Limits, TensorStream};

fn chunk(data: &[u8], compression: u8) -> Vec<u8> {
    let mut bytes = vec![10, u8::try_from(data.len()).unwrap()];
    bytes.extend_from_slice(data);
    bytes.extend_from_slice(&[16, compression]);
    bytes
}

#[test]
fn stream_tensors_checks_count_order_shape_and_expansion() {
    // feature: stream-tensors
    let mut stream = TensorStream::new(Limits::default()).unwrap();
    // Tensor(shape=[2], dtype=float32, chunk_count=2).
    assert!(stream.descriptor(&[10, 1, 2, 16, 2, 32, 2]).unwrap().is_none());
    assert!(stream.chunk(&chunk(&1f32.to_le_bytes(), 0)).unwrap().is_none());
    let tensor = stream.chunk(&chunk(&2f32.to_le_bytes(), 0)).unwrap().unwrap();
    assert_eq!(tensor.shape(), &[2]);
    assert_eq!(tensor.bytes(), [1f32.to_le_bytes(), 2f32.to_le_bytes()].concat());
    assert!(stream.finish().is_ok());
}

#[test]
fn stream_tensors_refuses_malformed_and_out_of_order_payloads() {
    // feature: stream-tensors
    for descriptor in [vec![], vec![10, 1, 2, 16, 99, 32, 1], vec![10, 1, 2, 16, 2, 32, 0], vec![10, 1, 2, 16, 2, 40, 1]] {
        let mut stream = TensorStream::new(Limits::default()).unwrap();
        assert_eq!(stream.descriptor(&descriptor).unwrap_err(), CodecError::BadReply);
    }
    let mut stream = TensorStream::new(Limits::default()).unwrap();
    assert_eq!(stream.chunk(&chunk(&[], 0)).unwrap_err(), CodecError::BadReply);
    assert_eq!(stream.finish().unwrap_err(), CodecError::BadReply);
    let mut stream = TensorStream::new(Limits::default()).unwrap();
    stream.descriptor(&[10, 1, 2, 16, 2, 32, 2]).unwrap();
    assert_eq!(stream.finish().unwrap_err(), CodecError::BadReply);
}

#[test]
fn stream_tensors_refuses_wire_and_shape_limits_before_decode() {
    // feature: stream-tensors
    let limits = Limits { message_bytes: 6, ..Limits::default() };
    let mut stream = TensorStream::new(limits).unwrap();
    assert_eq!(stream.descriptor(&[10, 1, 2, 16, 2, 32, 2]).unwrap_err(), CodecError::BadReply);
    let limits = Limits { expanded_bytes: 4, ..Limits::default() };
    let mut stream = TensorStream::new(limits).unwrap();
    assert_eq!(stream.descriptor(&[10, 1, 2, 16, 2, 32, 2]).unwrap_err(), CodecError::BadReply);
}

#[test]
fn stream_tensors_caps_zstd_output_and_window() {
    // feature: stream-tensors
    let compressed = zstd::stream::encode_all(&[0u8; 1024][..], 1).unwrap();
    let mut stream = TensorStream::new(Limits::default()).unwrap();
    stream.descriptor(&[10, 1, 1, 16, 2, 32, 1]).unwrap();
    assert_eq!(stream.chunk(&chunk(&compressed, 1)).unwrap_err(), CodecError::BadReply);
    let data = 7f32.to_le_bytes();
    let compressed = zstd::stream::encode_all(&data[..], 1).unwrap();
    let mut stream = TensorStream::new(Limits::default()).unwrap();
    stream.descriptor(&[10, 1, 1, 16, 2, 32, 1]).unwrap();
    assert_eq!(stream.chunk(&chunk(&compressed, 1)).unwrap().unwrap().bytes(), data);
    assert!(stream.finish().is_ok());
}
