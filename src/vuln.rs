use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Gripes {
    pub matches: Vec<Match>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub vulnerability: Vulnerability,
}

#[derive(Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,
}
