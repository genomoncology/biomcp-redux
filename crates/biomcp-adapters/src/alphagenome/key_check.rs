//! Exercise AlphaGenome authentication with metadata and a checked terminal status.
use biomcp_fetch::{
    BadReply, Leg, RequestError,
    grpc::{GrpcEndpoint, StreamDecoder},
};
use prost::Message;

const METADATA: GrpcEndpoint = super::SPEC
    .grpc("/google.gdm.gdmscience.alphagenome.v1main.DnaModelService/GetMetadata")
    .checked("alphagenome.key-check/1", biomcp_fetch::Freshness::Never)
    .invalid_key_message(invalid_key_message);

fn invalid_key_message(message: &[u8]) -> bool {
    matches!(
        message,
        b"API key not valid. Please pass a valid API key."
            | b"API%20key%20not%20valid.%20Please%20pass%20a%20valid%20API%20key."
    )
}

pub(crate) fn check(leg: &Leg<'_>) -> Result<(), RequestError> {
    let request = super::proto::MetadataRequest {
        organism: super::proto::Organism::HomoSapiens as i32,
    };
    leg.grpc_checked(&METADATA, &request.encode_to_vec(), |_| Ok(Decoder(false)))
}

struct Decoder(bool);

impl StreamDecoder for Decoder {
    type Reply = ();

    fn message(&mut self, message: &[u8]) -> Result<(), BadReply> {
        let metadata = super::proto::MetadataResponse::decode(message).map_err(|_| BadReply)?;
        self.0 |= !metadata.output_metadata.is_empty();
        Ok(())
    }

    fn finish(self) -> Result<(), BadReply> {
        self.0.then_some(()).ok_or(BadReply)
    }
}
