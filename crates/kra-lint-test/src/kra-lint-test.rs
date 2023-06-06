use std::error::Error;
use std::process::Command;

use diff::Result as DiffResult;

fn main() -> Result<(), Box<dyn Error>> {
    let test_directories = glob::glob("tests/*/")?
        .map(|glob_res| glob_res.map(|path_buf| path_buf.canonicalize()))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;

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

        let diff_buffers = |name, expected, current| {
            let diff_lines = diff::lines(expected, current)
                .iter()
                .filter_map(|diff_result| match diff_result {
                    DiffResult::Left(line) => Some(format!("-{}", line)),
                    DiffResult::Right(line) => Some(format!("+{}", line)),
                    DiffResult::Both(_, _) => None,
                })
                .collect::<Vec<_>>();

            if !diff_lines.is_empty() {
                println!("[{}] {}", name, test_directory.file_name().unwrap().to_str().unwrap());
                for diff_line in diff_lines {
                    println!("{}", diff_line);
                }
                println!();
            }
        };

        diff_buffers("STDOUT", &expected_stdout, &current_stdout);
        diff_buffers("STDERR", &expected_stderr, &current_stderr);
        diff_buffers("STATUS", &expected_status, &current_status);
    }

    Ok(())
}
