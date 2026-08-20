use url::Url;

use crate::{ontology::Ontology, space::Space};

pub mod ontology;
pub mod rid;
pub mod space;

struct Deployment {
    url: Url,
    ontologies: Vec<Ontology>,
    spaces: Vec<Space>,
}
