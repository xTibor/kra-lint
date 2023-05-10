use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult, LintStringMatchExpression};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassDocumentName {
    pub document_name: LintStringMatchExpression,
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

        Ok(results)
    }
}
