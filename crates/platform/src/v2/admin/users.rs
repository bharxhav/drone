use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
fn json<T: serde::Serialize>(v: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(v)?)
}
#[derive(Debug)]
pub struct Users<'c> {
    transport: &'c Transport,
}
impl<'c> Users<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.transport
            .send_no_content(Method::DELETE, &format!("v2/admin/users/{id}"), &[], None)
            .await
    }
    pub async fn get(&self, id: &str, status: Option<UserStatus>) -> Result<User> {
        let q = status
            .as_ref()
            .map(|v| vec![("status", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::GET, &format!("v2/admin/users/{id}"), &q, None)
            .await
    }
    pub async fn get_batch(
        &self,
        body: Vec<GetUsersBatchRequestElement>,
    ) -> Result<GetUsersBatchResponse> {
        let b = json(body)?;
        self.transport
            .send_json(Method::POST, "v2/admin/users/getBatch", &[], Some(&b))
            .await
    }
    pub async fn get_current(&self) -> Result<User> {
        self.transport
            .send_json(Method::GET, "v2/admin/users/getCurrent", &[], None)
            .await
    }
    pub async fn get_markings(&self, id: &str) -> Result<GetUserMarkingsResponse> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/users/{id}/getMarkings"),
                &[],
                None,
            )
            .await
    }
    pub async fn list(
        &self,
        include: Option<UserStatus>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListUsersResponse> {
        let size = page_size.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref v) = include {
            q.push(("include", v.as_str()))
        }
        if let Some(ref v) = size {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        self.transport
            .send_json(Method::GET, "v2/admin/users", &q, None)
            .await
    }
    pub async fn profile_picture(&self, id: &str) -> Result<bytes::Bytes> {
        self.transport
            .send_binary(
                Method::GET,
                &format!("v2/admin/users/{id}/profilePicture"),
                &[],
            )
            .await
    }
    pub async fn revoke_all_tokens(&self, id: &str) -> Result<()> {
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/users/{id}/revokeAllTokens"),
                &[],
                None,
            )
            .await
    }
    pub async fn search(&self, request: SearchUsersRequest) -> Result<SearchUsersResponse> {
        let b = json(request)?;
        self.transport
            .send_json(Method::POST, "v2/admin/users/search", &[], Some(&b))
            .await
    }
}
#[derive(Debug)]
pub struct UserProviderInfo<'c> {
    transport: &'c Transport,
}
impl<'c> UserProviderInfo<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, id: &str) -> Result<super::models::UserProviderInfo> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/users/{id}/providerInfo"),
                &[],
                None,
            )
            .await
    }
    pub async fn replace(
        &self,
        id: &str,
        request: ReplaceProviderInfoRequest,
    ) -> Result<super::models::UserProviderInfo> {
        let b = json(request)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/admin/users/{id}/providerInfo"),
                &[],
                Some(&b),
            )
            .await
    }
}
#[derive(Debug)]
pub struct GroupMemberships<'c> {
    transport: &'c Transport,
}
impl<'c> GroupMemberships<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn list(
        &self,
        id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
        transitive: Option<bool>,
    ) -> Result<ListGroupMembershipsResponse> {
        let size = page_size.map(|v| v.to_string());
        let tr = transitive.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref v) = size {
            q.push(("pageSize", v.as_str()))
        }
        if let Some(v) = page_token {
            q.push(("pageToken", v))
        }
        if let Some(ref v) = tr {
            q.push(("transitive", v.as_str()))
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/users/{id}/groupMemberships"),
                &q,
                None,
            )
            .await
    }
}
