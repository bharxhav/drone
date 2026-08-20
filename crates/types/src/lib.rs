use url::Url;

use crate::{ontology::Ontology, space::Space};

pub mod ontology;
pub mod rid;
pub mod space;

pub struct Deployment {
    pub url: Url,
    pub ontologies: Vec<Ontology>,
    pub spaces: Vec<Space>,
}
