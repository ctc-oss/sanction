use std::fs::File;
use std::io::{prelude::*, BufReader};

use glob::glob;
use serde::Deserialize;

pub fn load_allow_list(file: &File) -> Vec<Allowed> {
    let r = BufReader::new(file);

    r.lines()
        .map(|r| r.unwrap())
        .filter(|s| not_ignored(s))
        .map(|v| Allowed {
            id: v,
            by: "txt".to_string(),
        })
        .collect()
}

fn not_ignored(s: &str) -> bool {
    !ignored(s)
}

fn ignored(s: &str) -> bool {
    s.trim().is_empty() || s.trim_start().starts_with('#')
}

#[derive(Deserialize, Debug)]
pub struct Vuln {
    #[serde(rename = "vulnerability")]
    id: String,
}

pub struct Allowed {
    pub id: String,
    pub by: String,
}

#[derive(Deserialize, Debug)]
struct Allowlist {
    image_name: String,
    image_tag: String,
    image_parent_name: String,
    image_parent_tag: String,
    whitelisted_vulnerabilities: Vec<Vuln>,
}

impl Allowlist {
    fn image(&self) -> String {
        format!("{}:{}", self.image_name, self.image_tag)
    }
    fn image_parent(&self) -> Option<String> {
        match (&self.image_parent_name, &self.image_parent_tag) {
            (i, t) if !i.is_empty() && !t.is_empty() => Some(i.to_string()),
            _ => None,
        }
    }
}

fn vuln_to_allow(v: &Vuln, g: &Allowlist) -> Allowed {
    Allowed {
        id: v.id.clone(),
        by: g.image(),
    }
}

pub fn walk_allowlists(repo: &str, name: &str, ext: &str) -> Result<Vec<Allowed>, String> {
    match glob(format!("{}/{}/*.{}", repo, name, ext).as_str()) {
        Ok(mut e) => {
            let path = e.next().expect("image not found").unwrap();

            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            let r: serde_json::error::Result<Allowlist> = serde_json::from_reader(reader);
            match r {
                Ok(al) => match al.image_parent() {
                    Some(parent) => {
                        let mut x = vec![];
                        x.append(&mut walk_allowlists(repo, &parent, ext).unwrap());
                        x.append(
                            &mut al
                                .whitelisted_vulnerabilities
                                .iter()
                                .map(|v| vuln_to_allow(v, &al))
                                .collect(),
                        );
                        Ok(x)
                    }
                    None => Ok(al
                        .whitelisted_vulnerabilities
                        .iter()
                        .map(|v| vuln_to_allow(v, &al))
                        .collect()),
                },
                Err(e) => Err(format!("ERROR: {}", e)),
            }
        }
        _ => Err(String::from("Error _")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VULN_STR: &str = r#"{
        "vulnerability": "CCE-12345-6",
        "vuln_description": "description-text",
        "vuln_source": "OpenSCAP",
        "status": "approved",
        "approved_date": "1/1/2020",
        "approved_by": "foo@bar.com",
        "justification": "justification-text"
      }"#;

    const GL_STR: &str = r#"{
        "image_name": "image/name",
        "image_tag": "1.0",
        "image_parent_name": "redhat/ubi/ubi8",
        "image_parent_tag": "8.3",
        "container_owner": "fiz@baz.com",
        "approval_status": "notapproved",
        "authorized_approvers": [
            "foo@bar.com"
        ],
        "whitelisted_vulnerabilities": [
              {
                "vulnerability": "CCE-12345-6",
                "vuln_description": "description-text",
                "vuln_source": "OpenSCAP",
                "status": "approved",
                "approved_date": "1/1/2020",
                "approved_by": "foo@bar.com",
                "justification": "justification-text"
              }
        ]
    }"#;

    #[test]
    fn deserilize_list() {
        let al: Allowlist = serde_json::from_str(GL_STR).unwrap();
        println!("{}", al.whitelisted_vulnerabilities.first().unwrap().id);
    }

    #[test]
    fn deserilize_vuln() {
        let v: Vuln = serde_json::from_str(VULN_STR).unwrap();
        println!("{}", v.id);
    }
}
