use std::process::ExitCode;

use camino::Utf8PathBuf;
use clap::Parser;
use kra_lint_impl::LintConfig;

#[derive(Parser, Debug)]
pub struct Args {
    /// Source config file path
    pub source_config: Utf8PathBuf,

    /// Destination config file path
    pub destination_config: Utf8PathBuf,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let lint_config = LintConfig::load_from_path(&args.source_config).expect("Failed to load source config file");
    lint_config.save_to_path(&args.destination_config).expect("Failed to save destination config file");

    ExitCode::SUCCESS
}
