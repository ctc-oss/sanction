use std::str::FromStr;

#[derive(PartialEq, PartialOrd, Debug)]
enum Severity {
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
            _ => Err(String::from("invalid severity"))
        }
    }
}
