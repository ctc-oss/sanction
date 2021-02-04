use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use super::severity;

#[derive(Serialize, Deserialize)]
pub struct Grype {
    pub matches: Vec<Match>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub vulnerability: Vulnerability,
    pub artifact: Artifact,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowlist_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vulnerability {
    pub id: String,
    pub severity: severity::Severity,
    pub links: Vec<String>,
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_in_version: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub version: String,
}

impl fmt::Display for Artifact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
}
