use std::fs::File;
use std::io::{self};
use std::str::FromStr;

use clap::Clap;

use crate::allow::load_allow_list;

mod allow;
mod severity;
mod vuln;

/// allowlisting for grype scans
#[derive(Clap)]
#[clap(version = "v0.0.0")]
struct Opts {
    /// Path to allowlist
    #[clap(short('l'), long, default_value = "allow.txt")]
    allowlist: String,

    /// Output mode
    #[clap(short, long, default_value = "remove")]
    output: OutputMode,
}

#[derive(PartialEq, Debug)]
enum OutputMode {
    Remove,
    Tag,
}

impl FromStr for OutputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "remove" => Ok(OutputMode::Remove),
            "tag" => Ok(OutputMode::Tag),
            _ => Err(String::from("invalid mode"))
        }
    }
}


fn main() {
    let opts: Opts = Opts::parse();
    println!("mode: {:?}", opts.output);

    let f = File::open(opts.allowlist).unwrap();
    let a = load_allow_list(&f);
    println!("allowlist contains {} entries", a.len());

    let stdin = io::stdin();
    let gripes: vuln::Gripes = serde_json::from_reader(stdin).unwrap();

    let pre = gripes.matches.len();
    let post = gripes.matches.iter().filter(|v| !a.contains(&v.vulnerability.id)).count();

    println!("sanctioned {} vulnerabilities", pre - post);
}
