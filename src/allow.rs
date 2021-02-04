use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn load_allow_list(file: &File) -> Vec<String> {
    let r = BufReader::new(file);

    r.lines()
        .map(|r| r.unwrap())
        .filter(not_ignored)
        .collect()
}

fn not_ignored(s: &String) -> bool {
    !ignored(s)
}

fn ignored(s: &String) -> bool {
    s.trim().is_empty() || s.trim_start().starts_with('#')
}
