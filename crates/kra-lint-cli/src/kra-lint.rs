use std::process::ExitCode;

use camino::Utf8PathBuf;
use clap::Parser;

use kra_lint_impl::{LintConfigCollection, LintOutputFormat};

#[derive(Parser, Debug)]
pub struct Args {
    /// Config file path
    #[arg(long, short = 'C', value_name = "PATH", env = "KRALINT_CONFIG_PATH")]
    pub config_paths: Vec<Utf8PathBuf>,

    /// Output format
    #[arg(long, short = 'F', value_name = "FORMAT", env = "KRALINT_OUTPUT_FORMAT")]
    pub output_format: Option<LintOutputFormat>,

    /// Document paths
    pub paths: Vec<Utf8PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let lint_config_collection = {
        let mut lint_config_collection = LintConfigCollection::default();
        let mut config_paths = args.config_paths.clone();

        if config_paths.is_empty() {
            let default_config_path = Utf8PathBuf::from(".kra-lint");

            if default_config_path.is_file() {
                config_paths.push(default_config_path);
            }
        }

        if config_paths.is_empty() {
            eprintln!("kra-lint: No config files were found");
            return ExitCode::FAILURE;
        } else {
            for lint_config_path in config_paths {
                eprintln!("kra-lint: Using config file \"{}\"", lint_config_path);

                if let Err(err) = lint_config_collection.load_config(&lint_config_path) {
                    eprintln!("kra-lint: {}", err);
                    return ExitCode::FAILURE;
                }
            }
        }

        lint_config_collection
    };

    let lint_message_collection = lint_config_collection.lint_paths(&args.paths);

    if lint_message_collection.is_empty() {
        ExitCode::SUCCESS
    } else {
        let output_format = args.output_format.unwrap_or(LintOutputFormat::PlainText);
        let mut output_writer = std::io::stdout();

        lint_message_collection.write_output(&mut output_writer, output_format).expect("Failed to write output");

        ExitCode::FAILURE
    }
}
