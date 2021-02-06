use std::fs::File;
use std::io::{self};
use std::str::FromStr;

use clap::Clap;

use crate::allow::load_allow_list;
use crate::severity::Severity;
use std::ops::Deref;

mod allow;
mod grype;
mod markdown;
mod severity;

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

    /// Allow mode
    #[clap(short, long, default_value = "tag")]
    mode: AllowMode,

    /// Output mode
    #[clap(short, long, default_value = "json")]
    output: OutputMode,

    /// Pretty printing for markdown and json
    #[clap(long)]
    pretty: bool,
}

#[derive(PartialEq, Debug)]
enum AllowMode {
    Tag,
    Remove,
}
impl FromStr for AllowMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tag" => Ok(AllowMode::Tag),
            "remove" => Ok(AllowMode::Remove),
            _ => Err(String::from("invalid allow mode (remove | tag)")),
        }
    }
}

#[derive(PartialEq, Debug)]
enum OutputMode {
    Json,
    Md,
}

impl FromStr for OutputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputMode::Json),
            "md" => Ok(OutputMode::Md),
            _ => Err(String::from("invalid mode (json | md)")),
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let f = File::open(opts.allowlist).unwrap();
    let l = load_allow_list(&f);

    let stdin = io::stdin();
    let grype: grype::Scan = serde_json::from_reader(stdin).unwrap();

    let min_severity = opts.severity.unwrap_or(Severity::Unknown);
    let filtered = grype.matches.iter();

    let filtered: Vec<&grype::Match> = filtered
        .filter(|m| m.vulnerability.severity >= min_severity)
        .collect();

    let filtered: Vec<&grype::Match> = if opts.mode == AllowMode::Remove {
        filtered
            .iter()
            .map(|x| x.deref())
            .filter(|m| !l.contains(&m.vulnerability.id))
            .collect()
    } else {
        filtered.iter().map(|x| x.deref()).collect()
    };

    let output = match opts.output {
        OutputMode::Md if opts.pretty => markdown::pretty_table(filtered, l),
        OutputMode::Md => markdown::markdown_table(filtered, l),
        _ if opts.pretty => serde_json::to_string_pretty(&filtered).unwrap(),
        _ => serde_json::to_string(&filtered).unwrap(),
    };

    println!("{}", output);
}
