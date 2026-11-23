//! Exercise AlphaGenome authentication with metadata and a checked terminal status.
use biomcp_fetch::{
    BadReply, Leg, RequestError,
    grpc::{GrpcEndpoint, StreamDecoder},
};
use prost::Message;

const METADATA: GrpcEndpoint = super::SPEC
    .grpc("/google.gdm.gdmscience.alphagenome.v1main.DnaModelService/GetMetadata")
    .checked("alphagenome.key-check/1", biomcp_fetch::Freshness::Never)
    .google_key_check();

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
