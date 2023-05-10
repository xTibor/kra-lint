use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassProhibitCompositions {}

impl LintPass for LintPassProhibitCompositions {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            if let Some(composition_container) =
                kra_archive.main_doc.image.composition_container.as_ref()
            {
                for composition in &composition_container.compositions {
                    results.push(format!(
                        "Prohibited use of compositions (composition name: \"{}\")",
                        composition.name
                    ));
                }
            }
        }

        Ok(results)
    }
}
