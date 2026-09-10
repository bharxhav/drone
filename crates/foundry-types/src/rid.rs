use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct Rid {
    value: String,
    service: String,
    instance: String,
    resource_type: String,
    locator: String,
}

impl Rid {
    pub fn new(value: String) -> Option<Self> {
        let mut segments = value.split('.');

        if segments.next()? != "ri" {
            return None;
        }

        let service = segments.next()?.to_owned();
        let instance = segments.next()?.to_owned();
        let resource_type = segments.next()?.to_owned();
        let locator_segments = segments.collect::<Vec<_>>();

        if service.is_empty()
            || resource_type.is_empty()
            || locator_segments.is_empty()
            || locator_segments.iter().any(|segment| segment.is_empty())
        {
            return None;
        }

        let locator = locator_segments.join(".");

        Some(Self {
            value,
            service,
            instance,
            resource_type,
            locator,
        })
    }
}

impl fmt::Display for Rid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

impl TryFrom<String> for Rid {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value.clone()).ok_or(value)
    }
}

impl From<Rid> for String {
    fn from(rid: Rid) -> Self {
        rid.value
    }
}

#[cfg(test)]
mod tests {
    use super::Rid;

    #[test]
    fn parses_standard_rid() {
        assert!(Rid::new("ri.ontology.main.object-type.abc".into()).is_some());
    }

    #[test]
    fn parses_empty_instance() {
        for value in [
            "ri.magritte..source.123e4567-e89b-12d3-a456-426614174000",
            "ri.evals..evaluation-suite.123e4567-e89b-12d3-a456-426614174000",
            "ri.branch..branch.123e4567-e89b-12d3-a456-426614174000",
            "ri.widgetregistry..widget-set.123e4567-e89b-12d3-a456-426614174000",
        ] {
            assert!(Rid::new(value.into()).is_some());
        }
    }

    #[test]
    fn parses_dotted_locator() {
        assert!(Rid::new("ri.foundry.main.branch.dataset-id.bWFzdGVy".into()).is_some());
    }

    #[test]
    fn rejects_invalid_rids() {
        for value in [
            "ontology.main.object-type.abc",
            "ri..main.object-type.abc",
            "ri.ontology.main..abc",
            "ri.ontology.main.object-type",
            "ri.ontology.main.object-type.",
            "ri.ontology.main.object-type..abc",
        ] {
            assert!(Rid::new(value.into()).is_none());
        }
    }
}
