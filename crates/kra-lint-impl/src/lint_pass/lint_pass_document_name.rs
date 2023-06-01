use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config_fields::StringMatchExpression;
use crate::lint_output::lint_metadata_macros::{meta_expected, meta_found};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassDocumentName {
    document_name: StringMatchExpression,
}

impl LintPass for LintPassDocumentName {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_document_name = &kra_archive.main_doc.image.name;

            if kra_document_name.is_empty() {
                #[rustfmt::skip]
                lint_messages.push(
                    "Missing document name",
                    &[
                        meta_expected!(self.document_name),
                    ],
                );
            } else if !self.document_name.matches(kra_document_name) {
                #[rustfmt::skip]
                lint_messages.push(
                    "Incorrect document name",
                    &[
                        meta_expected!(self.document_name),
                        meta_found!(kra_document_name),
                    ],
                );
            }
        }

        Ok(())
    }
}
