use crate::{error::Result, transport::Transport};
use bytes::Bytes;
use reqwest::Method;
#[derive(Debug)]
pub struct ExperimentArtifactTables<'c> {
    transport: &'c Transport,
}
impl<'c> ExperimentArtifactTables<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    fn path(m: &str, e: &str, n: &str, k: &str) -> String {
        format!("v2/models/{m}/experiments/{e}/artifactTables/{n}/{k}")
    }
    pub async fn json(
        &self,
        m: &str,
        e: &str,
        n: &str,
        offset: Option<u32>,
        page_size: Option<u32>,
        preview: Option<&str>,
    ) -> Result<Bytes> {
        let o = offset.map(|v| v.to_string());
        let p = page_size.map(|v| v.to_string());
        let mut q = vec![];
        if let Some(ref v) = o {
            q.push(("offset", v.as_str()))
        }
        if let Some(ref v) = p {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = preview {
            q.push(("preview", v))
        }
        self.transport
            .send_binary(Method::GET, &Self::path(m, e, n, "json"), &q)
            .await
    }
    pub async fn parquet(&self, m: &str, e: &str, n: &str, preview: Option<&str>) -> Result<Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &Self::path(m, e, n, "parquet"),
                &preview.map(|v| vec![("preview", v)]).unwrap_or_default(),
            )
            .await
    }
}
