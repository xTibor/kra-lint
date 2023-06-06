use std::error::Error;
use std::process::Command;

use diff::Result as DiffResult;

fn main() -> Result<(), Box<dyn Error>> {
    let test_pass_directories = glob::glob("tests/*/")?
        .map(|glob_res| glob_res.map(|path_buf| path_buf.canonicalize()))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;

    for test_pass_directory in test_pass_directories {
        std::env::set_current_dir(&test_pass_directory)?;

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

        let current_stdout = std::str::from_utf8(&kra_lint_output.stdout)?;
        let current_stderr = std::str::from_utf8(&kra_lint_output.stderr)?;

        let diff_buffers = |name, expected, current| {
            println!("{}", name);
            for diff in diff::lines(expected, current) {
                match diff {
                    DiffResult::Left(l) => println!("-{}", l),
                    DiffResult::Right(r) => println!("+{}", r),
                    _ => {}
                }
            }
            println!("---");
            println!();
        };

        println!("{}", test_pass_directory.display());
        println!();
        diff_buffers("STDOUT", &expected_stdout, current_stdout);
        diff_buffers("STDERR", &expected_stderr, current_stderr);
    }

    Ok(())
}
