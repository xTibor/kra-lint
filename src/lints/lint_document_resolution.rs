use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassDocumentResolution {
    pub resolution: f64,
}

impl LintPass for LintPassDocumentResolution {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            let kra_resolution_x = kra_archive.main_doc.image.x_res;
            let kra_resolution_y = kra_archive.main_doc.image.y_res;

            if (kra_resolution_x != self.resolution)
                || (kra_resolution_y != self.resolution)
            {
                results.push(format!(
                    "Incorrect document resolution (expected: {}x{}dpi, found: {}x{}dpi)",
                    self.resolution, self.resolution, kra_resolution_x, kra_resolution_y
                ));
            }
        }

        results
    }
}
