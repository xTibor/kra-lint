use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult, LintStringMatchExpression};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassFileName {
    pub file_name: LintStringMatchExpression,
}

impl LintPass for LintPassFileName {
    fn lint(
        &self,
        kra_archive: &KraArchive,
        results: &mut Vec<String>,
    ) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_file_name = kra_archive
                .zip_path
                .file_name()
                .expect("Failed to get file name");

            if !self.file_name.matches(kra_file_name) {
                results.push(format!(
                    "Incorrect file name (expected: {}, found: \"{}\")",
                    self.file_name, kra_file_name,
                ));
            }
        }

        Ok(())
    }
}
