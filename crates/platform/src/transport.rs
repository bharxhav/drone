pub mod pagination;
pub mod request;
pub mod response;
pub mod retry;

use crate::auth::Auth;
use crate::error::{ApiError, Error, Result};
use bytes::Bytes;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Client as HttpClient, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use url::Url;

pub use request::RequestBody;
pub use response::{ByteStream, SseEvent, SseStream};

/// Internal HTTP transport shared across all namespace handles.
#[derive(Debug, Clone)]
pub struct Transport {
    pub(crate) http: HttpClient,
    pub(crate) base_url: Url,
    pub(crate) auth: Auth,
}

impl Transport {
    pub fn new(base_url: Url, auth: Auth) -> Self {
        let http = HttpClient::builder()
            .user_agent("drone-platform-sdk/0.1")
            .build()
            .expect("failed to build reqwest client");

        Self {
            http,
            base_url,
            auth,
        }
    }

    /// Send a JSON request and decode the response.
    pub async fn send_json<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        let body = body.cloned().map(RequestBody::Json);
        let response = self
            .send(method, path, query, HeaderMap::new(), body)
            .await?;
        self.handle_response(response).await
    }

    /// Send a request expecting no content (204).
    pub async fn send_no_content(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&serde_json::Value>,
    ) -> Result<()> {
        let body = body.cloned().map(RequestBody::Json);
        self.send(method, path, query, HeaderMap::new(), body)
            .await
            .map(|_| ())
    }

    /// Send a request and return raw bytes (binary download).
    pub async fn send_binary(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<bytes::Bytes> {
        self.send_binary_with(method, path, query, HeaderMap::new(), None)
            .await
    }

    /// Send an authenticated request with arbitrary headers and body.
    pub async fn send(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        headers: HeaderMap,
        body: Option<RequestBody>,
    ) -> Result<Response> {
        let request = self.request(method, path, query, headers, body)?;
        let response = request.send().await?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(self.parse_error(response).await)
        }
    }

    /// Send an authenticated request and buffer the response bytes.
    pub async fn send_binary_with(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        headers: HeaderMap,
        body: Option<RequestBody>,
    ) -> Result<Bytes> {
        Ok(self
            .send(method, path, query, headers, body)
            .await?
            .bytes()
            .await?)
    }

    /// Send an authenticated request and return response bytes as they arrive.
    pub async fn send_stream(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        headers: HeaderMap,
        body: Option<RequestBody>,
    ) -> Result<ByteStream> {
        Ok(ByteStream::new(
            self.send(method, path, query, headers, body).await?,
        ))
    }

    /// Send an authenticated request and parse its server-sent events.
    pub async fn send_sse(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        mut headers: HeaderMap,
        body: Option<RequestBody>,
    ) -> Result<SseStream> {
        headers
            .entry(ACCEPT)
            .or_insert(HeaderValue::from_static("text/event-stream"));
        Ok(SseStream::new(
            self.send(method, path, query, headers, body).await?,
        ))
    }

    fn request(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        headers: HeaderMap,
        body: Option<RequestBody>,
    ) -> Result<RequestBuilder> {
        let url = self
            .base_url
            .join(path)
            .map_err(|error| Error::Config(error.to_string()))?;
        let mut request = self
            .http
            .request(method, url)
            .header(AUTHORIZATION, self.auth.header_value())
            .headers(headers)
            .query(query);
        if let Some(body) = body {
            if matches!(body, RequestBody::Json(_)) {
                request = request.header(CONTENT_TYPE, "application/json");
            }
            request = body.apply(request);
        }
        Ok(request)
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: Response) -> Result<T> {
        if resp.status().is_success() {
            let body = resp.text().await?;
            serde_json::from_str(&body).map_err(Error::from)
        } else {
            Err(self.parse_error(resp).await)
        }
    }

    async fn parse_error(&self, resp: Response) -> Error {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();

        let api_err = match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(json) => ApiError {
                status,
                error_name: json
                    .get("errorName")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                error_code: json
                    .get("errorCode")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                error_instance_id: json
                    .get("errorInstanceId")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                error_description: json
                    .get("errorDescription")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                parameters: json
                    .get("parameters")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null),
            },
            Err(_) => ApiError {
                status,
                error_name: None,
                error_code: None,
                error_instance_id: None,
                error_description: Some(body),
                parameters: serde_json::Value::Null,
            },
        };

        Error::Api(Box::new(api_err))
    }
}
