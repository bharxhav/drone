use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;

fn json<T: serde::Serialize>(value: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(value)?)
}

#[derive(Debug)]
pub struct Groups<'c> {
    transport: &'c Transport,
}
impl<'c> Groups<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn create(&self, request: CreateGroupRequest) -> Result<Group> {
        let b = json(request)?;
        self.transport
            .send_json(Method::POST, "v2/admin/groups", &[], Some(&b))
            .await
    }
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.transport
            .send_no_content(Method::DELETE, &format!("v2/admin/groups/{id}"), &[], None)
            .await
    }
    pub async fn get(&self, id: &str) -> Result<Group> {
        self.transport
            .send_json(Method::GET, &format!("v2/admin/groups/{id}"), &[], None)
            .await
    }
    pub async fn get_batch(
        &self,
        body: Vec<GetGroupsBatchRequestElement>,
    ) -> Result<GetGroupsBatchResponse> {
        let b = json(body)?;
        self.transport
            .send_json(Method::POST, "v2/admin/groups/getBatch", &[], Some(&b))
            .await
    }
    pub async fn list(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListGroupsResponse> {
        let size = page_size.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref value) = size {
            q.push(("pageSize", value.as_str()));
        }
        if let Some(value) = page_token {
            q.push(("pageToken", value));
        }
        self.transport
            .send_json(Method::GET, "v2/admin/groups", &q, None)
            .await
    }
    pub async fn list_current(&self, preview: Option<bool>) -> Result<ListCurrentGroupsResponse> {
        let p = preview.map(|v| v.to_string());
        let q = p
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::GET, "v2/admin/groups/listCurrent", &q, None)
            .await
    }
    pub async fn replace(&self, id: &str, request: ReplaceGroupRequest) -> Result<Group> {
        let b = json(request)?;
        self.transport
            .send_json(Method::PUT, &format!("v2/admin/groups/{id}"), &[], Some(&b))
            .await
    }
    pub async fn search(&self, request: SearchGroupsRequest) -> Result<SearchGroupsResponse> {
        let b = json(request)?;
        self.transport
            .send_json(Method::POST, "v2/admin/groups/search", &[], Some(&b))
            .await
    }
}

#[derive(Debug)]
pub struct GroupMembers<'c> {
    transport: &'c Transport,
}
impl<'c> GroupMembers<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn add(&self, id: &str, request: AddGroupMembersRequest) -> Result<()> {
        let b = json(request)?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/groups/{id}/groupMembers/add"),
                &[],
                Some(&b),
            )
            .await
    }
    pub async fn list(
        &self,
        id: &str,
        include_expirations: Option<bool>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        transitive: Option<bool>,
    ) -> Result<ListGroupMembersResponse> {
        let size = page_size.map(|v| v.to_string());
        let ie = include_expirations.map(|v| v.to_string());
        let tr = transitive.map(|v| v.to_string());
        let mut q = Vec::new();
        if let Some(ref v) = ie {
            q.push(("includeExpirations", v.as_str()))
        }
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
                &format!("v2/admin/groups/{id}/groupMembers"),
                &q,
                None,
            )
            .await
    }
    pub async fn remove(&self, id: &str, principal_ids: Vec<PrincipalId>) -> Result<()> {
        let b = json(PrincipalIdsRequest { principal_ids })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/admin/groups/{id}/groupMembers/remove"),
                &[],
                Some(&b),
            )
            .await
    }
}

#[derive(Debug)]
pub struct GroupMembershipExpirationPolicies<'c> {
    transport: &'c Transport,
}
impl<'c> GroupMembershipExpirationPolicies<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, id: &str) -> Result<GroupMembershipExpirationPolicy> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/groups/{id}/membershipExpirationPolicy"),
                &[],
                None,
            )
            .await
    }
    pub async fn replace(
        &self,
        id: &str,
        request: ReplaceGroupMembershipExpirationPolicyRequest,
    ) -> Result<GroupMembershipExpirationPolicy> {
        let b = json(request)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/admin/groups/{id}/membershipExpirationPolicy"),
                &[],
                Some(&b),
            )
            .await
    }
}

#[derive(Debug)]
pub struct GroupProviderInfo<'c> {
    transport: &'c Transport,
}
impl<'c> GroupProviderInfo<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn get(&self, id: &str) -> Result<super::models::GroupProviderInfo> {
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/groups/{id}/providerInfo"),
                &[],
                None,
            )
            .await
    }
    pub async fn replace(
        &self,
        id: &str,
        request: ReplaceProviderInfoRequest,
    ) -> Result<super::models::GroupProviderInfo> {
        let b = json(request)?;
        self.transport
            .send_json(
                Method::PUT,
                &format!("v2/admin/groups/{id}/providerInfo"),
                &[],
                Some(&b),
            )
            .await
    }
}
