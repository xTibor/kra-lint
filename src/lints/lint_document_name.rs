use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult, StringMatchExpression};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassDocumentName {
    pub document_name: StringMatchExpression,
}

impl LintPass for LintPassDocumentName {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            let kra_document_name = &kra_archive.main_doc.image.name;

            if kra_document_name.is_empty() {
                results.push("Missing document name".to_owned());
            } else if !self.document_name.matches(kra_document_name) {
                results.push(format!(
                    "Incorrect document name (expected: {}, found: \"{}\")",
                    self.document_name, kra_document_name,
                ));
            }
        }

        results
    }
}
