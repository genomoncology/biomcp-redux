//! Let tonic own gRPC framing while preserving bounded opaque protobuf bytes.
use prost::bytes::{Buf, BufMut};
use tonic::codec::{BufferSettings, Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};

#[derive(Clone, Copy)]
pub(super) struct RawCodec {
    pub(super) limit: usize,
}
impl Codec for RawCodec {
    type Encode = prost::bytes::Bytes;
    type Decode = Vec<u8>;
    type Encoder = Self;
    type Decoder = Self;
    fn encoder(&mut self) -> Self {
        *self
    }
    fn decoder(&mut self) -> Self {
        *self
    }
}
impl Encoder for RawCodec {
    type Item = prost::bytes::Bytes;
    type Error = tonic::Status;
    fn encode(
        &mut self,
        item: prost::bytes::Bytes,
        dst: &mut EncodeBuf<'_>,
    ) -> Result<(), Self::Error> {
        if item.len() > self.limit {
            return Err(tonic::Status::out_of_range("bounded message"));
        }
        dst.put_slice(&item);
        Ok(())
    }
    fn buffer_settings(&self) -> BufferSettings {
        BufferSettings::new(self.limit.clamp(1, 1024), 1024)
    }
}
impl Decoder for RawCodec {
    type Item = Vec<u8>;
    type Error = tonic::Status;
    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        if src.remaining() > self.limit {
            return Err(tonic::Status::out_of_range("bounded message"));
        }
        Ok(Some(src.copy_to_bytes(src.remaining()).to_vec()))
    }
    fn buffer_settings(&self) -> BufferSettings {
        BufferSettings::new(self.limit.clamp(1, 1024), 1024)
    }
}
