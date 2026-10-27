//! Feature alphagenome-score-protocol: preserve the ruled request and admit complete scorer streams.
use biomcp_fetch::grpc::{Limits, StreamDecoder, alphagenome::ScoreRequest};
type Check = Result<(), Box<dyn std::error::Error + Send + Sync>>;
fn request(position: i64) -> Result<ScoreRequest, biomcp_fetch::InputError> {
    ScoreRequest::human_legacy(
        biodata::Build::GRCh38,
        "7".parse().map_err(|_| biomcp_fetch::InputError::Path)?,
        position,
        "A",
        "T",
    )
}
fn number(tag: u8, mut value: u64) -> Vec<u8> {
    let mut bytes = vec![tag << 3];
    while value > 127 {
        bytes.push((value as u8 & 127) | 128);
        value >>= 7;
    }
    bytes.push(value as u8);
    bytes
}
fn field(tag: u8, data: &[u8]) -> Vec<u8> {
    let mut bytes = number(tag, data.len() as u64);
    if let Some(first) = bytes.first_mut() {
        *first |= 2;
    }
    bytes.extend(data);
    bytes
}
fn variant(position: u64) -> Vec<u8> {
    [
        field(1, b"chr7"),
        number(2, position),
        field(3, b"A"),
        field(4, b"T"),
    ]
    .concat()
}
fn interval(start: u64, end: u64) -> Vec<u8> {
    [
        field(1, b"chr7"),
        number(2, start),
        number(3, end),
        number(4, 3),
    ]
    .concat()
}
fn descriptor(position: u64) -> Vec<u8> {
    let tensor = [field(1, &[1, 1, 1]), number(2, 2), number(4, 1)].concat();
    let metadata = [
        field(1, &variant(position)),
        field(2, &field(1, b"synthetic track")),
        field(3, &field(1, b"ENSG00000157764")),
    ]
    .concat();
    field(
        1,
        &field(1, &[field(1, &tensor), field(2, &metadata)].concat()),
    )
}
fn chunk() -> Vec<u8> {
    field(2, &field(1, &42f32.to_le_bytes()))
}
#[test]
fn canonical_request_preserves_legacy_interval_scorers_and_provider_default_checkpoint() -> Check {
    let gene = field(2, &number(1, 4));
    let splice = field(3, &number(2, 7));
    let center = field(1, &[number(1, 501), number(2, 1), number(3, 3)].concat());
    let expected = [
        field(1, &interval(140491191, 141015479)),
        field(2, &variant(140753336)),
        number(3, 9606),
        field(4, &gene),
        field(4, &splice),
        field(4, &center),
    ]
    .concat();
    assert_eq!(request(140753336)?.bytes(), expected);
    assert_eq!(request(140753336)?.build(), biodata::Build::GRCh38);
    assert_eq!(
        biomcp_fetch::grpc::alphagenome::SCORE_VARIANT,
        "/google.gdm.gdmscience.alphagenome.v1main.DnaModelService/ScoreVariant"
    );
    assert!(request(0).is_err());
    assert!(request(i64::MAX).is_err());
    assert!(
        ScoreRequest::human_legacy(biodata::Build::GRCh37, "7".parse()?, 140753336, "A", "T")
            .is_err()
    );
    assert!(
        ScoreRequest::human_legacy(biodata::Build::GRCh38, "7".parse()?, 140753336, "?", "T")
            .is_err()
    );
    // At the chromosome start the constant width follows the released request contract.
    assert!(request(1)?.bytes().starts_with(&field(
        1,
        &[field(1, b"chr7"), number(3, 524288), number(4, 3)].concat()
    )));
    Ok(())
}
#[test]
fn scorer_admission_requires_exact_outputs_variant_echo_and_descriptor_chunk_order() -> Check {
    let req = request(140753336)?;
    let mut complete = req
        .decoder(Limits::default())
        .map_err(|_| "limits rejected")?;
    for msg in [
        descriptor(140753336),
        chunk(),
        descriptor(140753336),
        chunk(),
        descriptor(140753336),
        chunk(),
    ] {
        complete
            .message(&msg)
            .map_err(|_| "valid stream rejected")?;
    }
    complete.finish().map_err(|_| "complete stream rejected")?;
    for messages in [
        vec![],
        vec![chunk()],
        vec![descriptor(140753336)],
        vec![descriptor(140753336), descriptor(140753336)],
        vec![descriptor(140753336), chunk()],
        vec![
            descriptor(140753336),
            chunk(),
            descriptor(140753336),
            chunk(),
            descriptor(140753336),
        ],
        vec![
            descriptor(140753336),
            chunk(),
            descriptor(140753336),
            chunk(),
            descriptor(140753336),
            chunk(),
            descriptor(140753336),
            chunk(),
        ],
        vec![
            descriptor(140753335),
            chunk(),
            descriptor(140753336),
            chunk(),
            descriptor(140753336),
            chunk(),
        ],
        vec![vec![10, 255]],
    ] {
        let mut decoder = req
            .decoder(Limits::default())
            .map_err(|_| "limits rejected")?;
        let accepted = messages
            .iter()
            .try_for_each(|msg| decoder.message(msg))
            .is_ok();
        assert!(!accepted || decoder.finish().is_err());
    }
    Ok(())
}
#[test]
fn scoring_stream_refuses_cumulative_envelope_and_tensor_limits() -> Check {
    let req = request(140753336)?;
    for limits in [
        Limits {
            message_bytes: 1,
            ..Default::default()
        },
        Limits {
            messages: 5,
            ..Default::default()
        },
        Limits {
            expanded_bytes: 2,
            ..Default::default()
        },
        Limits {
            wire_bytes: 20,
            ..Default::default()
        },
    ] {
        let mut decoder = req.decoder(limits).map_err(|_| "bounded limits rejected")?;
        let messages = [
            descriptor(140753336),
            chunk(),
            descriptor(140753336),
            chunk(),
            descriptor(140753336),
            chunk(),
        ];
        let accepted = messages
            .iter()
            .try_for_each(|msg| decoder.message(msg))
            .is_ok();
        assert!(!accepted || decoder.finish().is_err());
    }
    Ok(())
}
