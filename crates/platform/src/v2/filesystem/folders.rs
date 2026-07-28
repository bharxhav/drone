use crate::error::Result;
use crate::transport::Transport;
use reqwest::Method;

use super::models::*;

/// Resource handle for folder operations.
///
/// ```no_run
/// # use platform::v2::Client;
/// # use platform::auth::Auth;
/// # async fn example() -> platform::error::Result<()> {
/// # let client = Client::builder().hostname("x").auth(Auth::token("t")).build()?;
/// let folder = client.filesystem().folders().get("ri.compass.main.folder.abc").await?;
/// println!("{}", folder.display_name);
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Folders<'c> {
    transport: &'c Transport,
}

impl<'c> Folders<'c> {
    pub(crate) fn new(transport: &'c Transport) -> Self {
        Self { transport }
    }

    /// Get a folder by its RID.
    ///
    /// `GET /v2/filesystem/folders/{folderRid}`
    pub async fn get(&self, folder_rid: &str) -> Result<Folder> {
        let path = format!("v2/filesystem/folders/{folder_rid}");
        self.transport
            .send_json(Method::GET, &path, &[], None)
            .await
    }

    /// Create a new folder.
    ///
    /// `POST /v2/filesystem/folders`
    pub async fn create(&self, display_name: &str, parent_folder_rid: &str) -> Result<Folder> {
        let body = serde_json::to_value(CreateFolderRequest {
            parent_folder_rid: parent_folder_rid.to_owned(),
            display_name: display_name.to_owned(),
        })?;
        self.transport
            .send_json(Method::POST, "v2/filesystem/folders", &[], Some(&body))
            .await
    }

    /// Replace (update) a folder.
    ///
    /// `PUT /v2/filesystem/folders/{folderRid}`
    pub async fn replace(
        &self,
        folder_rid: &str,
        display_name: &str,
        parent_folder_rid: &str,
        preview: Option<bool>,
    ) -> Result<Folder> {
        let path = format!("v2/filesystem/folders/{folder_rid}");
        let body = serde_json::to_value(ReplaceFolderRequest {
            parent_folder_rid: parent_folder_rid.to_owned(),
            display_name: display_name.to_owned(),
        })?;
        let preview_value = preview.map(|value| value.to_string());
        let mut query = Vec::new();
        if let Some(value) = preview_value.as_deref() {
            query.push(("preview", value));
        }
        self.transport
            .send_json(Method::PUT, &path, &query, Some(&body))
            .await
    }

    /// List children of a folder (paginated).
    ///
    /// `GET /v2/filesystem/folders/{folderRid}/children`
    pub async fn children(
        &self,
        folder_rid: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListChildrenOfFolderResponse> {
        let path = format!("v2/filesystem/folders/{folder_rid}/children");
        let mut query: Vec<(&str, &str)> = Vec::new();
        let ps;
        if let Some(size) = page_size {
            ps = size.to_string();
            query.push(("pageSize", &ps));
        }
        if let Some(token) = page_token {
            query.push(("pageToken", token));
        }
        self.transport
            .send_json(Method::GET, &path, &query, None)
            .await
    }

    /// Fetch multiple folders in a single request (max 1000).
    ///
    /// `POST /v2/filesystem/folders/getBatch`
    pub async fn get_batch(
        &self,
        body: &[GetFoldersBatchRequestElement],
    ) -> Result<GetFoldersBatchResponse> {
        let body = serde_json::to_value(body)?;
        self.transport
            .send_json(
                Method::POST,
                "v2/filesystem/folders/getBatch",
                &[],
                Some(&body),
            )
            .await
    }
}
