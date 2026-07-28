use crate::{error::Result, transport::Transport};
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct ResourceRoles<'c> {
    transport: &'c Transport,
}

impl<'c> ResourceRoles<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    pub async fn list(
        &self,
        resource_rid: &str,
        include_inherited: Option<bool>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListResourceRolesResponse> {
        let inherited = include_inherited.map(|v| v.to_string());
        let size = page_size.map(|v| v.to_string());
        let mut query = Vec::new();
        if let Some(v) = inherited.as_deref() {
            query.push(("includeInherited", v));
        }
        if let Some(v) = size.as_deref() {
            query.push(("pageSize", v));
        }
        if let Some(v) = page_token {
            query.push(("pageToken", v));
        }
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/filesystem/resources/{resource_rid}/roles"),
                &query,
                None,
            )
            .await
    }
    pub async fn add(&self, resource_rid: &str, roles: Vec<ResourceRoleIdentifier>) -> Result<()> {
        let body = serde_json::to_value(AddResourceRolesRequest { roles })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/resources/{resource_rid}/roles/add"),
                &[],
                Some(&body),
            )
            .await
    }
    pub async fn remove(
        &self,
        resource_rid: &str,
        roles: Vec<ResourceRoleIdentifier>,
    ) -> Result<()> {
        let body = serde_json::to_value(RemoveResourceRolesRequest { roles })?;
        self.transport
            .send_no_content(
                Method::POST,
                &format!("v2/filesystem/resources/{resource_rid}/roles/remove"),
                &[],
                Some(&body),
            )
            .await
    }
}
