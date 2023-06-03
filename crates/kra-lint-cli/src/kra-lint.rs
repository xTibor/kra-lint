#![feature(error_iter)]

use std::error::Error;
use std::io::IsTerminal;
use std::process::ExitCode;

use camino::Utf8PathBuf;
use clap::Parser;

use kra_lint_impl::{LintConfigCollection, LintOutputFormat};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Config file paths
    #[arg(long, short = 'C', value_name = "PATH", env = "KRALINT_CONFIG_PATH")]
    config_paths: Vec<Utf8PathBuf>,

    /// Output format
    #[arg(long, short = 'F', value_name = "FORMAT", env = "KRALINT_OUTPUT_FORMAT")]
    output_format: Option<LintOutputFormat>,

    /// Document paths
    paths: Vec<Utf8PathBuf>,
}

fn main() -> ExitCode {
    match main_inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            for source in err.sources() {
                eprintln!("kra-lint: {}", source);
            }
            ExitCode::FAILURE
        }
    }
}

fn main_inner() -> Result<ExitCode, Box<dyn Error>> {
    let args = Args::try_parse()?;

    let lint_config_collection = {
        let mut lint_config_collection = LintConfigCollection::default();
        let mut config_paths = args.config_paths.clone();

        if config_paths.is_empty() {
            let search_root: Utf8PathBuf = std::env::current_dir()?.try_into()?;

            for search_directory in search_root.ancestors() {
                let default_config_path = search_directory.to_owned().join(".kra-lint");

                if default_config_path.is_file() {
                    config_paths.push(default_config_path);
                    break;
                }
            }
        }

        if config_paths.is_empty() {
            eprintln!("kra-lint: No config files were found");
            return Ok(ExitCode::FAILURE);
        } else {
            for lint_config_path in config_paths {
                eprintln!("kra-lint: Using config file \"{}\"", lint_config_path);
                lint_config_collection.load_config(&lint_config_path)?
            }
        }

        lint_config_collection
    };

    let lint_output_format = {
        let default_output_format =
            if std::io::stdout().is_terminal() { LintOutputFormat::PlainText } else { LintOutputFormat::Json };

        args.output_format.unwrap_or(default_output_format)
    };

    let lint_message_collection = lint_config_collection.lint_paths(&args.paths);

    lint_message_collection.write_output(&mut std::io::stdout(), lint_output_format)?;

    if lint_message_collection.is_empty() {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}
