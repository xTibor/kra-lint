use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassDocumentName {
    document_name: LintStringMatchExpression,
}

impl LintPass for LintPassDocumentName {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_document_name = &kra_archive.main_doc.image.name;

            if kra_document_name.is_empty() {
                lint_messages.push("Missing document name".to_owned());
            } else if !self.document_name.matches(kra_document_name) {
                lint_messages.push(format!(
                    "Incorrect document name (expected: {}, found: \"{}\")",
                    self.document_name, kra_document_name,
                ));
            }
        }

        Ok(())
    }
}
