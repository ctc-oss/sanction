use std::fs::File;
use std::io::{self};
use std::str::FromStr;

use clap::Clap;

use crate::allow::load_allow_list;
use crate::severity::Severity;

mod allow;
mod markdown;
mod severity;
mod vuln;

/// Basic allowlisting and formatting for grype scans
#[derive(Clap)]
#[clap(version = "v0.1.0")]
struct Opts {
    /// Path to allowlist
    #[clap(short('l'), long, default_value = "allow.txt")]
    allowlist: String,

    /// Minimum severity
    #[clap(short, long)]
    severity: Option<Severity>,

    /// Output mode
    #[clap(short, long, default_value = "remove")]
    output: OutputMode,
}

#[derive(PartialEq, Debug)]
enum OutputMode {
    Remove,
    Tag,
    Md,
}

impl FromStr for OutputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "remove" => Ok(OutputMode::Remove),
            "tag" => Ok(OutputMode::Tag),
            "md" => Ok(OutputMode::Md),
            _ => Err(String::from("invalid mode")),
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let f = File::open(opts.allowlist).unwrap();
    let l = load_allow_list(&f);

    let stdin = io::stdin();
    let grype: vuln::Grype = serde_json::from_reader(stdin).unwrap();

    let sss = opts.severity.unwrap_or(Severity::Unknown);
    let filtered: Vec<&vuln::Match> = grype
        .matches
        .iter()
        // todo;; improve the structure of the optional logic, for now disable the removal filter
        //.filter(|m| !l.contains(&m.vulnerability.id))
        .filter(|m| m.vulnerability.severity >= sss)
        .collect();

    match opts.output {
        OutputMode::Md => {
            markdown::dump_table(filtered, l)
        }
        _ => {
            let out = serde_json::to_string_pretty(&filtered).unwrap();
            println!("{}", out);
        }
    }
}
