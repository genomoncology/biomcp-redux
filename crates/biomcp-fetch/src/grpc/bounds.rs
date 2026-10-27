//! Declare the allocation envelope before any protobuf is decoded.
use super::CodecError;

/// Explicit per-stream bounds. Defaults support synthetic fixtures, not provider workloads.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    /// Maximum credential-free serialized request bytes.
    pub request_bytes: usize,
    /// Maximum bytes of one encoded protobuf message.
    pub message_bytes: usize,
    /// Maximum cumulative encoded protobuf bytes across a stream.
    pub wire_bytes: usize,
    /// Maximum number of descriptors plus chunks.
    pub messages: usize,
    /// Maximum dimensions in a tensor.
    pub rank: usize,
    /// Maximum value of each nonnegative dimension.
    pub dimension: usize,
    /// Maximum chunks announced by one descriptor.
    pub chunks: usize,
    /// Maximum cumulative expanded tensor bytes across a stream.
    pub expanded_bytes: usize,
    /// Maximum combined name/value bytes of response headers or trailers.
    pub metadata_bytes: usize,
    /// Maximum number of metadata fields in either response phase.
    pub metadata_fields: usize,
    /// Maximum calls waiting for the single in-flight stream.
    pub queued_calls: usize,
    /// Maximum live streams owned by the runtime.
    pub in_flight_streams: usize,
    /// Maximum HTTP/2 connection and stream receive window.
    pub h2_window: u32,
    /// Maximum ZSTD window as a base-two logarithm.
    pub zstd_window_log: u32,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            request_bytes: 64 * 1024,
            message_bytes: 1024 * 1024,
            wire_bytes: 8 * 1024 * 1024,
            messages: 256,
            rank: 8,
            dimension: 1024 * 1024,
            chunks: 64,
            expanded_bytes: 8 * 1024 * 1024,
            zstd_window_log: 23,
            metadata_bytes: 8192,
            metadata_fields: 64,
            queued_calls: 4,
            in_flight_streams: 2,
            h2_window: 65_535,
        }
    }
}
impl Limits {
    pub(super) fn check(self) -> Result<Self, CodecError> {
        let values = [
            self.request_bytes,
            self.message_bytes,
            self.wire_bytes,
            self.messages,
            self.rank,
            self.dimension,
            self.chunks,
            self.expanded_bytes,
            self.metadata_bytes,
            self.metadata_fields,
            self.queued_calls,
            self.in_flight_streams,
        ];
        if values.iter().any(|v| *v == 0 || *v > isize::MAX as usize)
            || self.queued_calls > tokio::sync::Semaphore::MAX_PERMITS
            || self.request_bytes > (u32::MAX as usize).saturating_sub(super::FRAME_BYTES)
            || self.h2_window == 0
            || self.h2_window > 16 * 1024 * 1024
            || self.metadata_bytes > u32::MAX as usize
            || self.dimension > i32::MAX as usize
            || !(10..=30).contains(&self.zstd_window_log)
        {
            return Err(CodecError::Limits);
        }
        Ok(self)
    }
    /// Reject an encoded request before ownership or queue admission.
    pub fn check_request(&self, bytes: &[u8]) -> Result<(), CodecError> {
        self.check()?;
        if bytes.len() > self.request_bytes {
            return Err(CodecError::BadReply);
        }
        Ok(())
    }
}
