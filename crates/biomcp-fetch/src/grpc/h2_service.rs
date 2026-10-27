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
            safe_headers(response.headers_mut(), self_limits)?;
            Ok(response.map(|stream| Receive {
                stream,
                limit,
                bytes: 0,
                phase: Phase::Data,
                limits: self_limits,
            }))
        })
    }
}
pub(super) struct Receive {
    stream: h2::RecvStream,
    limit: usize,
    bytes: usize,
    phase: Phase,
    limits: Limits,
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
                let checked = safe_headers(&mut headers, self.limits);
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
    Ok((Single { sender, limits }, connection))
}

fn unavailable() -> tonic::Status {
    tonic::Status::unavailable("stream unavailable")
}
fn safe_headers(headers: &mut http::HeaderMap, limits: Limits) -> Result<(), tonic::Status> {
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
    *headers = safe;
    Ok(())
}
