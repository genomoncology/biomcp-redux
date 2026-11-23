//! Adapt one explicit HTTP/2 connection to tonic without reconnect machinery.
use super::Limits;
use prost::bytes::Bytes;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tonic::codegen::{
    Body, Service,
    http::{self, Request, Response},
};

pub(super) struct Single {
    pub(super) sender: h2::client::SendRequest<Bytes>,
    pub(super) limits: Limits,
    google_key_check: bool,
    pub(super) state: std::sync::Arc<super::control::CallState>,
}
impl Service<Request<tonic::body::Body>> for Single {
    type Response = Response<Receive>;
    type Error = tonic::Status;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.sender.poll_ready(cx).map_err(|_| unavailable())
    }
    fn call(&mut self, request: Request<tonic::body::Body>) -> Self::Future {
        let sender = self.sender.clone();
        let self_limits = self.limits;
        let state = self.state.clone();
        let google_key_check = self.google_key_check;
        let limit = self_limits.wire_bytes;
        Box::pin(async move {
            let mut sender = sender.ready().await.map_err(|_| unavailable())?;
            let (head, mut body) = request.into_parts();
            let (response, mut send) = sender
                .send_request(Request::from_parts(head, ()), false)
                .map_err(|_| unavailable())?;
            while let Some(frame) =
                std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await
            {
                let frame = frame.map_err(|_| unavailable())?;
                if let Ok(mut data) = frame.into_data() {
                    while !data.is_empty() {
                        send.reserve_capacity(data.len());
                        let capacity = std::future::poll_fn(|cx| send.poll_capacity(cx))
                            .await
                            .ok_or_else(unavailable)?
                            .map_err(|_| unavailable())?;
                        let amount = capacity.min(data.len());
                        if amount == 0 {
                            continue;
                        }
                        send.send_data(data.split_to(amount), false)
                            .map_err(|_| unavailable())?;
                    }
                }
            }
            send.send_data(Bytes::new(), true)
                .map_err(|_| unavailable())?;
            let mut response = response.await.map_err(|_| unavailable())?;
            if response.status() != http::StatusCode::OK {
                return Err(http_failure(response.status().as_u16()));
            }
            safe_headers(response.headers_mut(), self_limits, google_key_check)?;
            terminal(response.headers(), &state);
            Ok(response.map(|stream| Receive {
                stream,
                limit,
                bytes: 0,
                phase: Phase::Data,
                limits: self_limits,
                google_key_check,
                state,
            }))
        })
    }
}
fn http_failure(status: u16) -> tonic::Status {
    use biomcp_types::Reason;
    let code = match crate::retry::status_reason(status) {
        Reason::NotFound => tonic::Code::NotFound,
        Reason::RateLimited => tonic::Code::ResourceExhausted,
        Reason::Unavailable => tonic::Code::Unavailable,
        _ => tonic::Code::PermissionDenied,
    };
    tonic::Status::new(code, "upstream HTTP failure")
}
pub(super) struct Receive {
    stream: h2::RecvStream,
    limit: usize,
    bytes: usize,
    phase: Phase,
    limits: Limits,
    google_key_check: bool,
    state: std::sync::Arc<super::control::CallState>,
}
enum Phase {
    Data,
    Trailers,
    Done,
}
impl Body for Receive {
    type Data = Bytes;
    type Error = tonic::Status;
    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        if matches!(self.phase, Phase::Done) {
            return Poll::Ready(None);
        }
        if matches!(self.phase, Phase::Data) {
            match self.stream.poll_data(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(Err(_))) => return Poll::Ready(Some(Err(unavailable()))),
                Poll::Ready(Some(Ok(data))) => {
                    self.bytes = self.bytes.saturating_add(data.len());
                    if self.bytes > self.limit {
                        return Poll::Ready(Some(Err(tonic::Status::data_loss(
                            "bounded response",
                        ))));
                    }
                    if self
                        .stream
                        .flow_control()
                        .release_capacity(data.len())
                        .is_err()
                    {
                        return Poll::Ready(Some(Err(unavailable())));
                    }
                    return Poll::Ready(Some(Ok(hyper::body::Frame::data(data))));
                }
                Poll::Ready(None) => self.phase = Phase::Trailers,
            }
        }
        let trailers = self.stream.poll_trailers(cx);
        if trailers.is_ready() {
            self.phase = Phase::Done;
        }
        match trailers {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(Some(mut headers))) => {
                let checked = safe_headers(&mut headers, self.limits, self.google_key_check);
                if checked.is_ok() {
                    terminal(&headers, &self.state);
                }
                Poll::Ready(Some(
                    checked.map(|()| hyper::body::Frame::trailers(headers)),
                ))
            }
            Poll::Ready(Ok(None)) => Poll::Ready(None),
            Poll::Ready(Err(_)) => Poll::Ready(Some(Err(unavailable()))),
        }
    }
}

pub(super) async fn establish(
    socket: super::connect::Socket,
    limits: Limits,
    state: std::sync::Arc<super::control::CallState>,
    google_key_check: bool,
) -> Result<
    (
        Single,
        h2::client::Connection<super::connect::Socket, Bytes>,
    ),
    h2::Error,
> {
    let (sender, connection) = h2::client::Builder::new()
        .initial_window_size(limits.h2_window)
        .initial_connection_window_size(limits.h2_window)
        .max_header_list_size(limits.metadata_bytes as u32)
        .header_table_size(limits.metadata_bytes as u32)
        .max_concurrent_streams(1)
        .max_send_buffer_size(limits.request_bytes.saturating_add(super::FRAME_BYTES))
        .handshake::<_, Bytes>(socket)
        .await?;
    Ok((
        Single {
            sender,
            limits,
            google_key_check,
            state,
        },
        connection,
    ))
}

fn unavailable() -> tonic::Status {
    tonic::Status::unavailable("stream unavailable")
}
fn safe_headers(
    headers: &mut http::HeaderMap,
    limits: Limits,
    google_key_check: bool,
) -> Result<(), tonic::Status> {
    let bad = || tonic::Status::data_loss("invalid metadata");
    if headers.len() > limits.metadata_fields {
        return Err(bad());
    }
    let mut total = 0usize;
    for (name, value) in headers.iter() {
        total = total
            .checked_add(name.as_str().len())
            .and_then(|v| v.checked_add(value.len()))
            .ok_or_else(bad)?;
        if total > limits.metadata_bytes {
            return Err(bad());
        }
    }
    let mut safe = http::HeaderMap::with_capacity(2);
    for name in ["content-type", "grpc-status"] {
        if let Some(value) = headers.get(name) {
            let valid = if name == "grpc-status" {
                value
                    .to_str()
                    .ok()
                    .and_then(|v| v.parse::<u8>().ok())
                    .is_some_and(|v| v <= 16)
            } else {
                matches!(
                    value.as_bytes(),
                    b"application/grpc" | b"application/grpc+proto"
                )
            };
            if !valid {
                return Err(bad());
            }
            safe.insert(name, value.clone());
        }
    }
    if headers
        .get("grpc-encoding")
        .is_some_and(|v| v != "identity")
    {
        return Err(bad());
    }
    // Keep the canonical authentication status and discard the publisher's message.
    if google_key_check
        && safe.get("grpc-status").is_some_and(|status| status == "3")
        && headers.get("grpc-message").is_some_and(|message| {
            matches!(
                message.as_bytes(),
                b"API key not valid. Please pass a valid API key."
                    | b"API%20key%20not%20valid.%20Please%20pass%20a%20valid%20API%20key."
            )
        })
    {
        safe.insert("grpc-status", http::HeaderValue::from_static("16"));
    }
    *headers = safe;
    Ok(())
}

fn terminal(headers: &http::HeaderMap, state: &super::control::CallState) {
    if let Some(code) = headers
        .get("grpc-status")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
    {
        state.set_terminal(code);
    }
}

#[cfg(test)]
mod tests {
    use super::{Limits, http, safe_headers};

    #[test]
    fn key_check_recognizes_only_the_publisher_invalid_key_message() -> Result<(), tonic::Status> {
        for (message, expected) in [
            ("API key not valid. Please pass a valid API key.", "16"),
            (
                "API%20key%20not%20valid.%20Please%20pass%20a%20valid%20API%20key.",
                "16",
            ),
            ("Malformed request", "3"),
        ] {
            let mut headers = http::HeaderMap::new();
            headers.insert("grpc-status", http::HeaderValue::from_static("3"));
            headers.insert("grpc-message", http::HeaderValue::from_static(message));
            safe_headers(&mut headers, Limits::DEFAULT, true)?;
            assert_eq!(
                headers
                    .get("grpc-status")
                    .and_then(|value| value.to_str().ok()),
                Some(expected),
                "only a recognized publisher error should change the status"
            );
            assert!(
                !headers.contains_key("grpc-message"),
                "provider message was retained"
            );
        }
        Ok(())
    }
}
