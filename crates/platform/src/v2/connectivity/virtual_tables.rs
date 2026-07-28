use reqwest::Method;

use super::models::*;
use crate::{error::Result, transport::Transport};

#[derive(Debug)]
pub struct VirtualTables<'c> {
    transport: &'c Transport,
}

impl<'c> VirtualTables<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    pub async fn create(
        &self,
        connection_rid: &str,
        request: CreateVirtualTableRequest,
    ) -> Result<VirtualTable> {
        let body = serde_json::to_value(request)?;
        self.transport
            .send_json(
                Method::POST,
                &format!("v2/connectivity/connections/{connection_rid}/virtualTables"),
                &[],
                Some(&body),
            )
            .await
    }
}
