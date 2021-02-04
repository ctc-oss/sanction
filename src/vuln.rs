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
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vulnerability {
    pub id: String,
    pub severity: severity::Severity,
    pub links: Vec<String>,
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_in_version: Option<String>
}
