use std::process::ExitCode;

use camino::Utf8PathBuf;
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
                eprintln!(
                    "kra-lint: Using config file \"{}\"",
                    lint_config_path
                );

                if let Err(err) =
                    lint_config_collection.load_config(&lint_config_path)
                {
                    eprintln!("kra-lint: {}", err)
                }
            }
        }

        lint_config_collection
    };

    let all_lint_messages = {
        let mut all_lint_messages = vec![];

        for kra_path in &args.paths {
            match KraArchive::from_path(kra_path) {
                Ok(kra_archive) => {
                    let mut lint_messages = vec![];

                    match lint_config_collection
                        .lint(&kra_archive, &mut lint_messages)
                    {
                        Ok(()) => all_lint_messages.extend(
                            lint_messages
                                .into_iter()
                                .map(|lint_message| (kra_path, lint_message)),
                        ),
                        Err(err) => {
                            all_lint_messages.push((kra_path, err.to_string()))
                        }
                    }
                }
                Err(err) => all_lint_messages.push((kra_path, err.to_string())),
            }
        }

        all_lint_messages.sort();
        all_lint_messages.dedup();
        all_lint_messages
    };

    if all_lint_messages.is_empty() {
        ExitCode::SUCCESS
    } else {
        for (kra_path, lint_message) in all_lint_messages {
            eprintln!("{}: {}", kra_path, lint_message);
        }
        ExitCode::FAILURE
    }
}
