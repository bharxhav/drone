use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

#[derive(Debug)]
pub struct Views<'c> {
    transport: &'c Transport,
}

impl<'c> Views<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    async fn backing(
        &self,
        method: Method,
        view_dataset_rid: &str,
        action: &str,
        backing_datasets: Vec<ViewBackingDataset>,
        branch: Option<&str>,
    ) -> Result<View> {
        let path = format!("v2/datasets/views/{view_dataset_rid}/{action}");
        let body = serde_json::to_value(AddBackingDatasetsRequest {
            branch: branch.map(str::to_owned),
            backing_datasets,
        })?;
        self.transport
            .send_json(method, &path, &[], Some(&body))
            .await
    }

    pub async fn add_backing_datasets(
        &self,
        view_dataset_rid: &str,
        backing_datasets: Vec<ViewBackingDataset>,
        branch: Option<&str>,
    ) -> Result<View> {
        self.backing(
            Method::POST,
            view_dataset_rid,
            "addBackingDatasets",
            backing_datasets,
            branch,
        )
        .await
    }

    pub async fn remove_backing_datasets(
        &self,
        view_dataset_rid: &str,
        backing_datasets: Vec<ViewBackingDataset>,
        branch: Option<&str>,
    ) -> Result<View> {
        self.backing(
            Method::POST,
            view_dataset_rid,
            "removeBackingDatasets",
            backing_datasets,
            branch,
        )
        .await
    }

    pub async fn replace_backing_datasets(
        &self,
        view_dataset_rid: &str,
        backing_datasets: Vec<ViewBackingDataset>,
        branch: Option<&str>,
    ) -> Result<View> {
        self.backing(
            Method::PUT,
            view_dataset_rid,
            "replaceBackingDatasets",
            backing_datasets,
            branch,
        )
        .await
    }

    pub async fn add_primary_key(
        &self,
        view_dataset_rid: &str,
        primary_key: ViewPrimaryKey,
        branch: Option<&str>,
    ) -> Result<View> {
        let path = format!("v2/datasets/views/{view_dataset_rid}/addPrimaryKey");
        let body = serde_json::to_value(AddPrimaryKeyRequest {
            branch: branch.map(str::to_owned),
            primary_key,
        })?;
        self.transport
            .send_json(Method::POST, &path, &[], Some(&body))
            .await
    }

    pub async fn create(
        &self,
        backing_datasets: Vec<ViewBackingDataset>,
        parent_folder_rid: &str,
        view_name: &str,
        branch: Option<&str>,
        primary_key: Option<ViewPrimaryKey>,
    ) -> Result<View> {
        let body = serde_json::to_value(CreateViewRequest {
            parent_folder_rid: parent_folder_rid.to_owned(),
            view_name: view_name.to_owned(),
            backing_datasets,
            branch: branch.map(str::to_owned),
            primary_key,
        })?;
        self.transport
            .send_json(Method::POST, "v2/datasets/views", &[], Some(&body))
            .await
    }

    pub async fn get(&self, view_dataset_rid: &str, branch: Option<&str>) -> Result<View> {
        let path = format!("v2/datasets/views/{view_dataset_rid}");
        let mut query = Vec::new();
        if let Some(v) = branch {
            query.push(("branch", v));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }
}
