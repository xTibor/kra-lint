#![feature(error_iter)]

use std::error::Error;
use std::process::{Command, ExitCode};

use diff::Result as DiffResult;

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
        .map(|glob_res| glob_res.map(|path_buf| path_buf.canonicalize()))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;

    let mut diff_found = false;

    for test_directory in test_directories {
        std::env::set_current_dir(&test_directory)?;

        let input_documents = glob::glob("*.kr[az]")?.collect::<Result<Vec<_>, _>>()?;

        let kra_lint_output = Command::new("cargo")
            .args(["run", "--bin", "kra-lint", "--quiet"])
            .args(["--"])
            .args(["-C", ".kra-lint"])
            .args(["-F", "plain-text"])
            .args(input_documents)
            .output()?;

        let expected_stdout = std::fs::read_to_string("expected.stdout")?;
        let expected_stderr = std::fs::read_to_string("expected.stderr")?;
        let expected_status = std::fs::read_to_string("expected.status")?;

        let current_stdout = String::from_utf8(kra_lint_output.stdout)?;
        let current_stderr = String::from_utf8(kra_lint_output.stderr)?;
        let current_status = format!("{}\n", kra_lint_output.status.to_string());

        for (name, expected, current) in &[
            ("STDOUT", &expected_stdout, &current_stdout),
            ("STDERR", &expected_stderr, &current_stderr),
            ("STATUS", &expected_status, &current_status),
        ] {
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

                println!("[{}] {}", name, test_directory.file_name().unwrap().to_str().unwrap());
                for diff_line in diff_lines {
                    println!("{}", diff_line);
                }
                println!();
            }
        }
    }

    if diff_found {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
