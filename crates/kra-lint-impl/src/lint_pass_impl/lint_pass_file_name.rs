use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_expected, meta_found};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassFileName {
    file_name: LintStringMatchExpression,
}

impl LintPass for LintPassFileName {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_file_name = kra_archive.zip_path.file_name().expect("Failed to get file name");

            if !self.file_name.matches(kra_file_name) {
                #[rustfmt::skip]
                lint_messages.push(
                    "Incorrect file name",
                    &[
                        meta_expected!(self.file_name),
                        meta_found!(kra_file_name),
                    ],
                );
            }
        }

        Ok(())
    }
}
