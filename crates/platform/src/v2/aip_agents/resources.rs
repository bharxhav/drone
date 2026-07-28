use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

fn query(
    preview: Option<bool>,
    page_size: Option<u32>,
    page_token: Option<&str>,
) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(value) = page_size {
        query.push(("pageSize".into(), value.to_string()));
    }
    if let Some(value) = page_token {
        query.push(("pageToken".into(), value.into()));
    }
    if let Some(value) = preview {
        query.push(("preview".into(), value.to_string()));
    }
    query
}

fn refs(query: &[(String, String)]) -> Vec<(&str, &str)> {
    query
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect()
}

#[derive(Debug)]
pub struct Agents<'c> {
    transport: &'c Transport,
}
impl<'c> Agents<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get(
        &self,
        agent_rid: &str,
        version: Option<&str>,
        preview: Option<bool>,
    ) -> Result<Agent> {
        let path = format!("v2/aipAgents/agents/{agent_rid}");
        let preview_value = preview.map(|value| if value { "true" } else { "false" });
        let mut query = Vec::new();
        if let Some(value) = version {
            query.push(("version", value));
        }
        if let Some(value) = preview_value {
            query.push(("preview", value));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn all_sessions(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<bool>,
    ) -> Result<AgentsSessionsPage> {
        let query = query(preview, page_size, page_token);
        let query = refs(&query);
        self.transport
            .send_json(Method::GET, "v2/aipAgents/agents/allSessions", &query, None)
            .await
    }
}

#[derive(Debug)]
pub struct AgentVersions<'c> {
    transport: &'c Transport,
}
impl<'c> AgentVersions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn get(
        &self,
        agent_rid: &str,
        version: &str,
        preview: Option<bool>,
    ) -> Result<AgentVersion> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/agentVersions/{version}");
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn list(
        &self,
        agent_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<bool>,
    ) -> Result<ListAgentVersionsResponse> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/agentVersions");
        let query = query(preview, page_size, page_token);
        let query = refs(&query);
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}

#[derive(Debug)]
pub struct Sessions<'c> {
    transport: &'c Transport,
}
impl<'c> Sessions<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn create(
        &self,
        agent_rid: &str,
        request: &CreateSessionRequest,
        preview: Option<bool>,
    ) -> Result<Session> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/sessions");
        let body = serde_json::to_value(request)?;
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_json(Method::POST, &path, &query, Some(&body))
            .await
    }

    pub async fn delete(
        &self,
        agent_rid: &str,
        session_rid: &str,
        preview: Option<bool>,
    ) -> Result<()> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/sessions/{session_rid}");
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_no_content(Method::DELETE, &path, &query, None)
            .await
    }

    pub async fn get(
        &self,
        agent_rid: &str,
        session_rid: &str,
        preview: Option<bool>,
    ) -> Result<Session> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/sessions/{session_rid}");
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn list(
        &self,
        agent_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        preview: Option<bool>,
    ) -> Result<ListSessionsResponse> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/sessions");
        let query = query(preview, page_size, page_token);
        let query = refs(&query);
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    pub async fn blocking_continue(
        &self,
        agent_rid: &str,
        session_rid: &str,
        request: &BlockingContinueSessionRequest,
        preview: Option<bool>,
    ) -> Result<SessionExchangeResult> {
        let path =
            format!("v2/aipAgents/agents/{agent_rid}/sessions/{session_rid}/blockingContinue");
        let body = serde_json::to_value(request)?;
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_json(Method::POST, &path, &query, Some(&body))
            .await
    }

    pub async fn cancel(
        &self,
        agent_rid: &str,
        session_rid: &str,
        request: &CancelSessionRequest,
        preview: Option<bool>,
    ) -> Result<CancelSessionResponse> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/sessions/{session_rid}/cancel");
        let body = serde_json::to_value(request)?;
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_json(Method::POST, &path, &query, Some(&body))
            .await
    }

    pub async fn rag_context(
        &self,
        agent_rid: &str,
        session_rid: &str,
        request: &GetRagContextForSessionRequest,
        preview: Option<bool>,
    ) -> Result<AgentSessionRagContextResponse> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/sessions/{session_rid}/ragContext");
        let body = serde_json::to_value(request)?;
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_json(Method::PUT, &path, &query, Some(&body))
            .await
    }

    pub async fn update_title(
        &self,
        agent_rid: &str,
        session_rid: &str,
        request: &UpdateSessionTitleRequest,
        preview: Option<bool>,
    ) -> Result<()> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/sessions/{session_rid}/updateTitle");
        let body = serde_json::to_value(request)?;
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_no_content(Method::PUT, &path, &query, Some(&body))
            .await
    }
}

#[derive(Debug)]
pub struct Contents<'c> {
    transport: &'c Transport,
}
impl<'c> Contents<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        agent_rid: &str,
        session_rid: &str,
        preview: Option<bool>,
    ) -> Result<Content> {
        let path = format!("v2/aipAgents/agents/{agent_rid}/sessions/{session_rid}/content");
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}

#[derive(Debug)]
pub struct SessionTraces<'c> {
    transport: &'c Transport,
}
impl<'c> SessionTraces<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(
        &self,
        agent_rid: &str,
        session_rid: &str,
        trace_id: &str,
        preview: Option<bool>,
    ) -> Result<SessionTrace> {
        let path = format!(
            "v2/aipAgents/agents/{agent_rid}/sessions/{session_rid}/sessionTraces/{trace_id}"
        );
        let query = query(preview, None, None);
        let query = refs(&query);
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}
