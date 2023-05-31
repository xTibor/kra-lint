use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::meta_comment;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassProhibitCompositions {}

impl LintPass for LintPassProhibitCompositions {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(composition_container) = kra_archive.main_doc.image.composition_container.as_ref() {
                for composition in composition_container.into_iter() {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Prohibited use of compositions",
                        &[
                            meta_comment!(format!("Composition name: \"{}\"", composition.name)),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
