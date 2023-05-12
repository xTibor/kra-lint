use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassProhibitCompositions {}

impl LintPass for LintPassProhibitCompositions {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(composition_container) = kra_archive.main_doc.image.composition_container.as_ref() {
                for composition in &composition_container.compositions {
                    lint_messages
                        .push(format!("Prohibited use of compositions (composition name: \"{}\")", composition.name));
                }
            }
        }

        Ok(())
    }
}
