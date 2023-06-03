#![feature(error_iter)]

use std::error::Error;
use std::process::ExitCode;

use camino::Utf8PathBuf;
use clap::Parser;
use kra_lint_impl::LintConfig;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Source config file path
    source_config: Utf8PathBuf,

    /// Destination config file path
    destination_config: Utf8PathBuf,
}

fn main() -> ExitCode {
    match main_inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            for source in err.sources() {
                eprintln!("kra-lint-convert: {}", source);
            }
            ExitCode::FAILURE
        }
    }
}

fn main_inner() -> Result<ExitCode, Box<dyn Error>> {
    let args = Args::try_parse()?;

    let lint_config = LintConfig::load_from_path(&args.source_config)?;
    lint_config.save_to_path(&args.destination_config)?;

    Ok(ExitCode::SUCCESS)
}
