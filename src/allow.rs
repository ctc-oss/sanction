use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn load_allow_list(file: &File) -> Vec<String> {
    let r = BufReader::new(file);

    r.lines()
        .map(|r| r.unwrap())
        .filter(|s| not_ignored(s))
        .collect()
}

fn not_ignored(s: &str) -> bool {
    !ignored(s)
}

fn ignored(s: &str) -> bool {
    s.trim().is_empty() || s.trim_start().starts_with('#')
}
