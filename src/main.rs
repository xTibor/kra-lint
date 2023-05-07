#![warn(clippy::pattern_type_mismatch)]

use std::fs;
use std::process::ExitCode;

use camino::Utf8PathBuf;
use clap::Parser;

mod lints;
mod models;

use crate::lints::{LintConfig, LintPass};
use crate::models::kra_archive::KraArchive;

#[derive(Parser, Debug)]
pub struct Args {
    /// Config file path
    #[arg(long, short = 'C', value_name = "PATH", env = "KRALINT_CONFIG_PATH")]
    pub config_path: Option<Utf8PathBuf>,

    /// Document paths
    pub paths: Vec<Utf8PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let lint_config_path =
        args.config_path.unwrap_or(Utf8PathBuf::from(".kra-lint"));

    let lint_config_str = fs::read_to_string(lint_config_path)
        .expect("Failed to read config file");

    let lint_config: LintConfig =
        toml::from_str(&lint_config_str).expect("Failed to parse config file");

    let mut lint_results = vec![];

    for path in &args.paths {
        match KraArchive::from_path(path) {
            Ok(kra_archive) => {
                lint_results.extend(
                    lint_config
                        .lint(&kra_archive)
                        .into_iter()
                        .map(|lint_message| (path, lint_message)),
                );
            }
            Err(err) => lint_results.push((path, err.to_string())),
        }
    }

    lint_results.sort();

    if lint_results.is_empty() {
        ExitCode::SUCCESS
    } else {
        for (path, lint_message) in lint_results {
            eprintln!("{}: {}", path, lint_message);
        }
        ExitCode::FAILURE
    }
}
