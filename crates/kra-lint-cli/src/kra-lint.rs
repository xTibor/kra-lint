#![warn(clippy::pattern_type_mismatch)]

use std::process::ExitCode;

use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;

use kra_lint_impl::{LintConfigCollection, LintPass};
use kra_parser::kra_archive::KraArchive;

#[derive(Parser, Debug)]
pub struct Args {
    /// Config file path
    #[arg(long, short = 'C', value_name = "PATH", env = "KRALINT_CONFIG_PATH")]
    pub config_paths: Vec<Utf8PathBuf>,

    /// Document paths
    pub paths: Vec<Utf8PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut lint_config_collection = LintConfigCollection::default();

    if args.config_paths.is_empty() {
        lint_config_collection.load_config(Utf8Path::new(".kra-lint"));
    } else {
        for lint_config_path in args.config_paths {
            lint_config_collection.load_config(&lint_config_path);
        }
    }

    let mut lint_results = vec![];

    for path in &args.paths {
        match KraArchive::from_path(path) {
            Ok(kra_archive) => {
                lint_results.extend(
                    lint_config_collection
                        .lint(&kra_archive)
                        .into_iter()
                        .map(|lint_message| (path, lint_message)),
                );
            }
            Err(err) => lint_results.push((path, err.to_string())),
        }
    }

    lint_results.sort();
    lint_results.dedup();

    if lint_results.is_empty() {
        ExitCode::SUCCESS
    } else {
        for (path, lint_message) in lint_results {
            eprintln!("{}: {}", path, lint_message);
        }
        ExitCode::FAILURE
    }
}
