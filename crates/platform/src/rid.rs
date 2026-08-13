use std::{borrow::Borrow, fmt, ops::Deref, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};

use crate::InvalidRid;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct Rid(String);

impl Rid {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Rid {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Rid {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Rid {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Display for Rid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Rid {
    type Err = InvalidRid;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(rest) = value.strip_prefix("ri.") else {
            return Err(InvalidRid::Prefix);
        };
        let Some(service_end_relative) = rest.find('.') else {
            return Err(InvalidRid::Structure);
        };
        let service_end = 3 + service_end_relative;
        let rest = &value[service_end + 1..];
        let Some(instance_end_relative) = rest.find('.') else {
            return Err(InvalidRid::Structure);
        };
        let instance_end = service_end + 1 + instance_end_relative;
        let service = &value[3..service_end];
        let instance = &value[service_end + 1..instance_end];
        let suffix = &value[instance_end + 1..];

        if service.is_empty() {
            return Err(InvalidRid::Service);
        }
        if suffix.is_empty() || suffix.split('.').any(str::is_empty) {
            return Err(InvalidRid::Structure);
        }
        if !service
            .split('.')
            .chain([instance])
            .chain(suffix.split('.'))
            .all(valid_component)
        {
            return Err(InvalidRid::Component);
        }

        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<String> for Rid {
    type Error = InvalidRid;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Rid {
    type Error = InvalidRid;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<Rid> for String {
    fn from(rid: Rid) -> Self {
        rid.into_string()
    }
}

impl PartialEq<str> for Rid {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Rid {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'de> Deserialize<'de> for Rid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

fn valid_component(component: &str) -> bool {
    component
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}
