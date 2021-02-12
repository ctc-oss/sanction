use std::fs::File;
use std::io::{self};
use std::ops::Deref;
use std::str::FromStr;

use clap::Clap;
use log::info;

use sanction::allow::{load_allow_list, walk_allowlists};
use sanction::grype;
use sanction::markdown;
use sanction::repo::latest;
use sanction::severity::Severity;

/// Basic allowlisting and formatting for grype scans
#[derive(Clap)]
#[clap(version = "v0.2.0")]
struct Opts {
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

    #[clap(subcommand)]
    source: ListSource,
}

#[derive(Clap)]
enum ListSource {
    /// fapolicyd service commands
    #[clap(version = "0.0.1")]
    Git(GitSubOpts),
    Txt(TxtSubOpts),
}

/// Git repo allowlist commands
#[derive(Clap)]
struct GitSubOpts {
    /// URL of allowlist repo
    #[clap()]
    url: String,

    /// Image name
    #[clap()]
    image: String,

    /// File extension of allowlists for globbing
    #[clap(short, long, default_value = "greylist")]
    ext: String,

    /// Path to repo
    #[clap(long)]
    repodir: Option<String>,
}

/// Txt file allowlist commands
#[derive(Clap)]
struct TxtSubOpts {
    /// Path to allowlist
    #[clap()]
    path: String,
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
    let allows = match opts.source {
        ListSource::Txt(opts) => {
            let f = File::open(opts.path).unwrap();
            load_allow_list(&f)
        }
        ListSource::Git(opts) => {
            let url = opts.url;
            let dir = opts.repodir;
            let (repo, sha) = latest(url.as_str(), &dir).unwrap();
            info!("using whitelist version {}", sha);

            let name = opts.image;
            let ext = opts.ext;
            walk_allowlists(repo.as_str(), name.as_str(), ext.as_str()).unwrap()
        }
    };
    let l: Vec<String> = allows.iter().map(|a| a.id.clone()).collect();

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
