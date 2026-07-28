use std::sync::Arc;

use url::Url;

use crate::auth::Auth;
use crate::error::Error;
use crate::transport::Transport;

use super::admin::Admin;
use super::aip_agents::AipAgents;
use super::audit::Audit;
use super::checkpoints::Checkpoints;
use super::connectivity::Connectivity;
use super::data_health::DataHealth;
use super::datasets::Datasets;
use super::filesystem::Filesystem;
use super::functions::Functions;
use super::language_models::LanguageModels;
use super::media_sets::MediaSets;
use super::models::Models;
use super::ontologies::Ontologies;
use super::orchestration::Orchestration;
use super::public_apis::PublicApis;
use super::sql_queries::SqlQueries;
use super::streams::Streams;
use super::third_party_applications::ThirdPartyApplications;
use super::widgets::Widgets;

/// The Foundry Platform v2 client.
///
/// Cheap to clone (internally reference-counted).
///
/// ```no_run
/// use platform::v2::Client;
/// use platform::auth::Auth;
///
/// # async fn example() -> platform::error::Result<()> {
/// let client = Client::builder()
///     .hostname("stack.palantirfoundry.com")
///     .auth(Auth::token("my-token"))
///     .build()?;
///
/// let folder = client.filesystem().folders();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    transport: Arc<Transport>,
}

/// Builder for constructing a [`Client`].
#[derive(Debug)]
pub struct ClientBuilder {
    hostname: Option<String>,
    auth: Option<Auth>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            hostname: None,
            auth: None,
        }
    }

    pub fn admin(&self) -> Admin<'_> {
        Admin::new(&self.transport)
    }

    pub fn aip_agents(&self) -> AipAgents<'_> {
        AipAgents::new(&self.transport)
    }

    pub fn audit(&self) -> Audit<'_> {
        Audit::new(&self.transport)
    }

    pub fn checkpoints(&self) -> Checkpoints<'_> {
        Checkpoints::new(&self.transport)
    }

    pub fn connectivity(&self) -> Connectivity<'_> {
        Connectivity::new(&self.transport)
    }

    pub fn data_health(&self) -> DataHealth<'_> {
        DataHealth::new(&self.transport)
    }

    pub fn datasets(&self) -> Datasets<'_> {
        Datasets::new(&self.transport)
    }

    pub fn filesystem(&self) -> Filesystem<'_> {
        Filesystem::new(&self.transport)
    }

    pub fn functions(&self) -> Functions<'_> {
        Functions::new(&self.transport)
    }

    pub fn language_models(&self) -> LanguageModels<'_> {
        LanguageModels::new(&self.transport)
    }

    pub fn media_sets(&self) -> MediaSets<'_> {
        MediaSets::new(&self.transport)
    }

    pub fn models(&self) -> Models<'_> {
        Models::new(&self.transport)
    }

    pub fn ontologies(&self) -> Ontologies<'_> {
        Ontologies::new(&self.transport)
    }

    pub fn orchestration(&self) -> Orchestration<'_> {
        Orchestration::new(&self.transport)
    }

    pub fn public_apis(&self) -> PublicApis<'_> {
        PublicApis::new(&self.transport)
    }

    pub fn sql_queries(&self) -> SqlQueries<'_> {
        SqlQueries::new(&self.transport)
    }

    pub fn streams(&self) -> Streams<'_> {
        Streams::new(&self.transport)
    }

    pub fn third_party_applications(&self) -> ThirdPartyApplications<'_> {
        ThirdPartyApplications::new(&self.transport)
    }

    pub fn widgets(&self) -> Widgets<'_> {
        Widgets::new(&self.transport)
    }
}

impl ClientBuilder {
    pub fn hostname(mut self, hostname: impl Into<String>) -> Self {
        self.hostname = Some(hostname.into());
        self
    }

    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let hostname = self
            .hostname
            .ok_or_else(|| Error::Config("hostname is required".into()))?;

        let auth = self
            .auth
            .ok_or_else(|| Error::Config("auth is required".into()))?;

        let base_url = Url::parse(&format!("https://{hostname}/api/"))
            .map_err(|e| Error::Config(format!("invalid hostname: {e}")))?;

        let transport = Transport::new(base_url, auth);

        Ok(Client {
            transport: Arc::new(transport),
        })
    }
}
