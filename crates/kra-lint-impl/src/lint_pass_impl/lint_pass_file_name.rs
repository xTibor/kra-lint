use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassFileName {
    file_name: LintStringMatchExpression,
}

impl LintPass for LintPassFileName {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_file_name = kra_archive.zip_path.file_name().expect("Failed to get file name");

            if !self.file_name.matches(kra_file_name) {
                lint_messages
                    .push(format!("Incorrect file name (expected: {}, found: \"{}\")", self.file_name, kra_file_name,));
            }
        }

        Ok(())
    }
}
