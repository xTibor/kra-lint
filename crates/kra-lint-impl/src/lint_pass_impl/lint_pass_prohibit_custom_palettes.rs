use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassProhibitCustomPalettes {}

impl LintPass for LintPassProhibitCustomPalettes {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            if let Some(kra_palette_container) =
                kra_archive.main_doc.image.palette_container.as_ref()
            {
                for kra_palette in &kra_palette_container.resources {
                    results.push(format!(
                        "Prohibited use of custom palettes (palette: \"{}\")",
                        kra_palette.name
                    ));
                }
            }
        }

        results
    }
}
