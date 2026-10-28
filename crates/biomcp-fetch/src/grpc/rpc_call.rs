//! Drive one bounded tonic stream to checked terminal status on a single connection.
use super::{connect, h2_service, raw_codec::RawCodec, worker::Command};
use biomcp_types::Reason;
use std::sync::mpsc::{SyncSender, TrySendError};
use tonic::codegen::tokio_stream;

pub(super) enum Event {
    Message(Vec<u8>),
    End(Result<(), Reason>),
}
pub(super) async fn execute(
    command: &Command,
    tls: std::sync::Arc<rustls::ClientConfig>,
) -> Result<(), Reason> {
    let socket = connect::open(&command.uri, command.proxy.as_ref(), tls, command.limits).await?;
    let (single, connection) = h2_service::establish(socket, command.limits, command.state.clone())
        .await
        .map_err(|_| Reason::Unavailable)?;
    let call = stream(command, single);
    tokio::pin!(call);
    tokio::pin!(connection);
    tokio::select! {
        result = &mut call => result,
        _ = &mut connection => call.await,
    }
}
async fn stream(command: &Command, single: h2_service::Single) -> Result<(), Reason> {
    let mut client = tonic::client::Grpc::with_origin(single, command.uri.clone())
        .max_decoding_message_size(command.limits.message_bytes)
        .max_encoding_message_size(command.limits.request_bytes);
    let mut request = tonic::Request::new(tokio_stream::once(command.body.clone()));
    request.set_timeout(
        command
            .deadline
            .end
            .saturating_duration_since(std::time::Instant::now()),
    );
    if let Some((name, value)) = &command.key {
        let name = tonic::metadata::MetadataKey::from_bytes(name.as_bytes())
            .map_err(|_| Reason::Rejected)?;
        let mut value: tonic::metadata::MetadataValue<_> =
            value.value().parse().map_err(|_| Reason::Rejected)?;
        value.set_sensitive(true);
        request.metadata_mut().insert(name, value);
    }
    if command.deadline.expired() {
        return Err(Reason::DeadlineExceeded);
    }
    let response = client
        .streaming(
            request,
            command.path.clone(),
            RawCodec {
                limit: command
                    .limits
                    .message_bytes
                    .max(command.limits.request_bytes),
            },
        )
        .await
        .map_err(status)?;
    let mut responses = response.into_inner();
    let mut messages = 0usize;
    while let Some(bytes) = responses.message().await.map_err(status)? {
        messages = messages.checked_add(1).ok_or(Reason::BadReply)?;
        if messages > command.limits.messages || reflected(command, &bytes) {
            return Err(Reason::BadReply);
        }
        deliver(&command.events, Event::Message(bytes)).await?;
    }
    // message() checks final gRPC status. Explicit trailers also refuse a missing terminal result.
    responses.trailers().await.map_err(status)?;
    Ok(())
}
fn reflected(command: &Command, bytes: &[u8]) -> bool {
    command.key.as_ref().is_some_and(|(_, key)| {
        !key.value().is_empty()
            && bytes
                .windows(key.value().len())
                .any(|part| part == key.value().as_bytes())
    })
}
pub(super) async fn deliver(sender: &SyncSender<Event>, mut event: Event) -> Result<(), Reason> {
    loop {
        match sender.try_send(event) {
            Ok(()) => return Ok(()),
            Err(TrySendError::Disconnected(_)) => return Err(Reason::DeadlineExceeded),
            Err(TrySendError::Full(value)) => event = value,
        }
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
}
fn status(status: tonic::Status) -> Reason {
    code_reason(status.code() as u8)
}
pub(super) fn code_reason(code: u8) -> Reason {
    match tonic::Code::from_i32(i32::from(code)) {
        tonic::Code::DeadlineExceeded | tonic::Code::Cancelled => Reason::DeadlineExceeded,
        tonic::Code::ResourceExhausted => Reason::RateLimited,
        tonic::Code::PermissionDenied | tonic::Code::Unauthenticated => Reason::Rejected,
        tonic::Code::InvalidArgument | tonic::Code::DataLoss | tonic::Code::Internal => {
            Reason::BadReply
        }
        tonic::Code::NotFound => Reason::NotFound,
        _ => Reason::Unavailable,
    }
}
