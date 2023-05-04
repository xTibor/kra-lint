use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassDocumentSize {
    pub width: usize,
    pub height: usize,
}

impl LintPass for LintPassDocumentSize {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            let kra_width = kra_archive.main_doc.image.width;
            let kra_height = kra_archive.main_doc.image.height;

            if (kra_width != self.width) || (kra_height != self.height) {
                results.push(format!(
                    "Incorrect document size (expected: {}x{}px, found: {}x{}px)",
                    self.width, self.height, kra_width, kra_height
                ));
            }
        }

        results
    }
}
