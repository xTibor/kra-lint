#![feature(error_iter)]

use std::error::Error;
use std::process::{Command, ExitCode};
use std::{env, fs, io, str};

use camino::Utf8PathBuf;
use derive_more::{Display, Error};
use diff::Result as DiffResult;

#[rustfmt::skip]
#[derive(Debug, Display, Error)]
enum TestError {
    #[display(fmt = "Failed to extract test name (\"{test_directory:}\")")]
    ExtractTestName {
        test_directory: Utf8PathBuf,
    },

    #[display(fmt = "Failed to read input arguments ({test_name:})")]
    InputArgs {
        test_name: String,
        source: io::Error,
    },

    #[display(fmt = "Failed to read expected standard output ({test_name:})")]
    ExpectedStdout {
        test_name: String,
        source: io::Error,
    },

    #[display(fmt = "Failed to read expected standard error ({test_name:})")]
    ExpectedStderr {
        test_name: String,
        source: io::Error,
    },

    #[display(fmt = "Failed to read expected process status ({test_name:})")]
    ExpectedStatus {
        test_name: String,
        source: io::Error,
    },
}

fn main() -> ExitCode {
    match main_inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            for source in err.sources() {
                eprintln!("kra-lint-test: {}", source);
            }
            ExitCode::FAILURE
        }
    }
}

fn main_inner() -> Result<ExitCode, Box<dyn Error>> {
    let test_directories = glob::glob("tests/*/")?
        .map(|glob_res| glob_res.map(|path_buf| path_buf.canonicalize().map(Utf8PathBuf::try_from)))
        .collect::<Result<Result<Result<Vec<Utf8PathBuf>, _>, _>, _>>()???;

    let mut diff_found = false;

    for test_directory in &test_directories {
        env::set_current_dir(test_directory)?;

        #[rustfmt::skip]
        let test_name = test_directory.file_name()
            .ok_or(TestError::ExtractTestName { test_directory: test_directory.clone() })?;

        let input_args = fs::read_to_string("kra-lint.args")
            .map_err(|source| TestError::InputArgs { test_name: test_name.to_owned(), source })?
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();

        let input_documents = glob::glob("*.kr[az]")?
            .map(|glob_res| glob_res.map(Utf8PathBuf::try_from))
            .collect::<Result<Result<Vec<Utf8PathBuf>, _>, _>>()??;

        let kra_lint_output = Command::new("cargo")
            .args(["run", "--bin", "kra-lint", "--quiet"])
            .args(["--"])
            .args(input_args)
            .args(input_documents)
            .output()?;

        let expected_stdout = fs::read("kra-lint.stdout")
            .map_err(|source| TestError::ExpectedStdout { test_name: test_name.to_owned(), source })?;

        let expected_stderr = fs::read("kra-lint.stderr")
            .map_err(|source| TestError::ExpectedStderr { test_name: test_name.to_owned(), source })?;

        let expected_status = fs::read("kra-lint.status")
            .map_err(|source| TestError::ExpectedStatus { test_name: test_name.to_owned(), source })?;

        let current_stdout = kra_lint_output.stdout;

        let current_stderr = kra_lint_output.stderr;

        let current_status = format!("{}\n", kra_lint_output.status).as_bytes().to_owned();

        for (name, expected, current) in &[
            ("STDOUT", &expected_stdout, &current_stdout),
            ("STDERR", &expected_stderr, &current_stderr),
            ("STATUS", &expected_status, &current_status),
        ] {
            match (str::from_utf8(expected), str::from_utf8(current)) {
                (Ok(expected), Ok(current)) => {
                    let diff_lines = diff::lines(expected, current)
                        .iter()
                        .filter_map(|diff_result| match diff_result {
                            DiffResult::Left(line) => Some(format!("-{}", line)),
                            DiffResult::Right(line) => Some(format!("+{}", line)),
                            DiffResult::Both(_, _) => None,
                        })
                        .collect::<Vec<_>>();

                    if !diff_lines.is_empty() {
                        diff_found = true;

                        println!("[{}] {}", name, test_name);
                        for diff_line in diff_lines {
                            println!("{}", diff_line);
                        }
                        println!();
                    }
                }
                _ => {
                    if current != expected {
                        diff_found = true;

                        println!("[{}] {}", name, test_name);
                        println!("Binary mismatch");
                        println!();
                    }
                }
            }
        }
    }

    if diff_found {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
