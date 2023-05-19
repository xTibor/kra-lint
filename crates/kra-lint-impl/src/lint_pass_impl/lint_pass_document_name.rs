use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::LintMessages;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassDocumentName {
    document_name: LintStringMatchExpression,
}

impl LintPass for LintPassDocumentName {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_document_name = &kra_archive.main_doc.image.name;

            if kra_document_name.is_empty() {
                lint_messages.push("Missing document name", format!("Expected: {}", self.document_name));
            } else if !self.document_name.matches(kra_document_name) {
                lint_messages.push(
                    "Incorrect document name",
                    format!("Expected: {}, Found: \"{}\"", self.document_name, kra_document_name.escape_debug()),
                );
            }
        }

        Ok(())
    }
}
