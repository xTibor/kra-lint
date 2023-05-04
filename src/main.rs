#![warn(clippy::pattern_type_mismatch)]

use clap::Parser;
use lints::LintPass;

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

mod lints;
mod models;

use crate::lints::LintConfig;
use crate::models::kra_archive::KraArchive;

#[derive(Parser, Debug)]
pub struct Args {
    /// Config file path
    #[arg(long, short = 'C', value_name = "PATH", env = "KRALINT_CONFIG_PATH")]
    pub config_path: Option<PathBuf>,

    /// Document paths
    pub paths: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let lint_config_path =
        args.config_path.unwrap_or(PathBuf::from(".kra-lint"));

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
            eprintln!("{}: {}", path.display(), lint_message);
        }
        ExitCode::FAILURE
    }
}
