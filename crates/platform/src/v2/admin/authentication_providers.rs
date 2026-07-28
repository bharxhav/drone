use super::models::*;
use crate::{error::Result, transport::Transport};
use reqwest::Method;
fn json<T: serde::Serialize>(v: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(v)?)
}
#[derive(Debug)]
pub struct AuthenticationProviders<'c> {
    transport: &'c Transport,
}
impl<'c> AuthenticationProviders<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }
    fn path(e: &str, p: &str) -> String {
        format!("v2/admin/enrollments/{e}/authenticationProviders/{p}")
    }
    pub async fn get(
        &self,
        e: &str,
        p: &str,
        preview: Option<bool>,
    ) -> Result<AuthenticationProvider> {
        let pv = preview.map(|v| v.to_string());
        let q = pv
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(Method::GET, &Self::path(e, p), &q, None)
            .await
    }
    pub async fn list(
        &self,
        e: &str,
        preview: Option<bool>,
    ) -> Result<ListAuthenticationProvidersResponse> {
        let pv = preview.map(|v| v.to_string());
        let q = pv
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::GET,
                &format!("v2/admin/enrollments/{e}/authenticationProviders"),
                &q,
                None,
            )
            .await
    }
    pub async fn preregister_group(
        &self,
        e: &str,
        p: &str,
        r: PreregisterGroupRequest,
        preview: Option<bool>,
    ) -> Result<PrincipalId> {
        let b = json(r)?;
        let pv = preview.map(|v| v.to_string());
        let q = pv
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::POST,
                &format!("{}/preregisterGroup", Self::path(e, p)),
                &q,
                Some(&b),
            )
            .await
    }
    pub async fn preregister_user(
        &self,
        e: &str,
        p: &str,
        r: PreregisterUserRequest,
        preview: Option<bool>,
    ) -> Result<PrincipalId> {
        let b = json(r)?;
        let pv = preview.map(|v| v.to_string());
        let q = pv
            .as_ref()
            .map(|v| vec![("preview", v.as_str())])
            .unwrap_or_default();
        self.transport
            .send_json(
                Method::POST,
                &format!("{}/preregisterUser", Self::path(e, p)),
                &q,
                Some(&b),
            )
            .await
    }
}
