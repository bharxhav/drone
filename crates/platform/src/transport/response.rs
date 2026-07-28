use bytes::Bytes;
use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::error::{Error, Result};

/// Streaming response bytes. Transport errors are reported as stream items.
pub struct ByteStream {
    inner: Pin<Box<dyn Stream<Item = std::result::Result<Bytes, reqwest::Error>> + Send>>,
}

impl std::fmt::Debug for ByteStream {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ByteStream").finish_non_exhaustive()
    }
}

impl ByteStream {
    pub(crate) fn new(response: reqwest::Response) -> Self {
        Self {
            inner: Box::pin(response.bytes_stream()),
        }
    }
}

impl Stream for ByteStream {
    type Item = Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner
            .as_mut()
            .poll_next(context)
            .map(|item| item.map(|result| result.map_err(Error::from)))
    }
}

/// One server-sent event. Unknown SSE fields are ignored.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SseEvent {
    pub event: Option<String>,
    pub data: String,
    pub id: Option<String>,
    pub retry: Option<u64>,
}

/// Parsed server-sent event stream.
#[derive(Debug)]
pub struct SseStream {
    bytes: ByteStream,
    buffer: Vec<u8>,
    complete: bool,
}

impl SseStream {
    pub(crate) fn new(response: reqwest::Response) -> Self {
        Self {
            bytes: ByteStream::new(response),
            buffer: Vec::new(),
            complete: false,
        }
    }

    fn take_event(&mut self, end: usize, separator_len: usize) -> Option<Result<SseEvent>> {
        let block = self.buffer.drain(..end).collect::<Vec<_>>();
        self.buffer.drain(..separator_len);
        parse_event(&block).transpose()
    }
}

impl Stream for SseStream {
    type Item = Result<SseEvent>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some((end, separator_len)) = event_boundary(&self.buffer) {
                if let Some(event) = self.take_event(end, separator_len) {
                    return Poll::Ready(Some(event));
                }
                continue;
            }

            if self.complete {
                if self.buffer.is_empty() {
                    return Poll::Ready(None);
                }
                let block = std::mem::take(&mut self.buffer);
                return Poll::Ready(parse_event(&block).transpose());
            }

            match Pin::new(&mut self.bytes).poll_next(context) {
                Poll::Ready(Some(Ok(bytes))) => self.buffer.extend_from_slice(&bytes),
                Poll::Ready(Some(Err(error))) => return Poll::Ready(Some(Err(error))),
                Poll::Ready(None) => self.complete = true,
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

fn event_boundary(buffer: &[u8]) -> Option<(usize, usize)> {
    let lf = buffer.windows(2).position(|window| window == b"\n\n");
    let crlf = buffer.windows(4).position(|window| window == b"\r\n\r\n");
    match (lf, crlf) {
        (Some(left), Some(right)) if left <= right => Some((left, 2)),
        (Some(_), Some(right)) => Some((right, 4)),
        (Some(left), None) => Some((left, 2)),
        (None, Some(right)) => Some((right, 4)),
        (None, None) => None,
    }
}

fn parse_event(block: &[u8]) -> Result<Option<SseEvent>> {
    let text = std::str::from_utf8(block)
        .map_err(|error| Error::Config(format!("invalid UTF-8 in SSE response: {error}")))?;
    let mut event = SseEvent::default();
    let mut data = Vec::new();
    let mut has_field = false;

    for line in text.lines() {
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.starts_with(':') {
            continue;
        }
        let (field, value) = line.split_once(':').unwrap_or((line, ""));
        let value = value.strip_prefix(' ').unwrap_or(value);
        match field {
            "event" => {
                event.event = Some(value.to_owned());
                has_field = true;
            }
            "data" => {
                data.push(value);
                has_field = true;
            }
            "id" if !value.contains('\0') => {
                event.id = Some(value.to_owned());
                has_field = true;
            }
            "retry" => {
                event.retry = value.parse().ok();
                has_field = true;
            }
            _ => {}
        }
    }

    event.data = data.join("\n");
    Ok(has_field.then_some(event))
}

#[cfg(test)]
mod tests {
    use super::{event_boundary, parse_event};

    #[test]
    fn parses_sse_fields_and_multiline_data() {
        let event = parse_event(b"id: 7\nevent: update\ndata: first\ndata: second\nretry: 1000")
            .unwrap()
            .unwrap();

        assert_eq!(event.id.as_deref(), Some("7"));
        assert_eq!(event.event.as_deref(), Some("update"));
        assert_eq!(event.data, "first\nsecond");
        assert_eq!(event.retry, Some(1000));
    }

    #[test]
    fn finds_crlf_event_boundary() {
        assert_eq!(event_boundary(b"data: value\r\n\r\nnext"), Some((11, 4)));
    }
}
