//! Publish whole RPC captures with ordered safe binary siblings.
use super::*;
use crate::grpc::spool::Spool;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Rpc {
    pub(crate) protocol: Protocol,
    pub(crate) messages: Vec<Message>,
    pub(crate) terminal: Option<u8>,
    pub(crate) interruption: Option<biomcp_types::Reason>,
    pub(crate) delay_ms: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum Protocol {
    #[serde(rename = "grpc/1")]
    GrpcV1,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Message {
    pub(crate) sidecar: String,
    pub(crate) delay_ms: u64,
}
impl Rpc {
    pub(super) fn valid(&self) -> bool {
        self.terminal.is_none_or(|code| code <= 16)
            && (self.terminal.is_some() || self.interruption.is_some())
            && !(self.terminal.is_some_and(|code| code != 0) && self.interruption.is_some())
    }
}
impl Recordings {
    pub(crate) fn append_rpc(
        &self,
        source: biomcp_types::Source,
        key: &[u8; 32],
        url: &str,
        post: &crate::prepared::PostMetadata,
        spool: &mut Spool,
        mut rpc: Rpc,
    ) -> Result<(), RecordError> {
        let Self::Capture(capture) = self else {
            return Ok(());
        };
        let mut sequences = capture.sequences.lock().map_err(|_| RecordError::Io)?;
        let recording = sequences
            .entry((source, *key))
            .or_insert_with(|| Recording {
                method: "POST".into(),
                source,
                url: url.into(),
                post: Some(post.clone()),
                exchanges: Vec::new(),
            });
        if recording.exchanges.len() >= MAX_EXCHANGES {
            return Err(RecordError::Limit);
        }
        let owner = path(&capture.dir, source, key);
        let stem = owner
            .file_stem()
            .and_then(|v| v.to_str())
            .ok_or(RecordError::Malformed)?;
        let binary_owner =
            owner.with_file_name(format!("{stem}-grpc-{}.json", recording.exchanges.len()));
        for index in 0..spool.messages.len() {
            let delay_ms = spool
                .messages
                .get(index)
                .ok_or(RecordError::Malformed)?
                .delay_ms;
            let sidecar = sidecar::publish(&binary_owner, index, &spool.message(index)?)?;
            rpc.messages.push(Message { sidecar, delay_ms });
        }
        recording.exchanges.push(Exchange {
            fetched: crate::cache::now(),
            status: 200,
            content_type: "application/grpc".into(),
            content_range: None,
            pagination: None,
            delay_ms: 0,
            retry_after_ms: None,
            body: Body::Text(String::new()),
            rpc: Some(rpc),
        });
        let result = publish(&owner, recording, MAX_FILE_BYTES);
        if result.is_err() {
            recording.exchanges.pop();
        }
        result
    }
    pub(crate) fn rpc_message(
        &self,
        source: biomcp_types::Source,
        key: &[u8; 32],
        message: &Message,
        limit: u64,
    ) -> Result<Vec<u8>, RecordError> {
        let Self::Replay { dir, .. } = self else {
            return Err(RecordError::Malformed);
        };
        sidecar::read(&path(dir, source, key), &message.sidecar, limit)
    }
}
