use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum Severity {
    Unknown,
    Negligible,
    Low,
    Medium,
    High,
    Critical,
}

impl FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "negligible" => Ok(Severity::Negligible),
            "low" => Ok(Severity::Low),
            "medium" => Ok(Severity::Medium),
            "high" => Ok(Severity::High),
            "critical" => Ok(Severity::Critical),
            _ => Err(String::from("invalid severity")),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
