use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::{LintMessages, LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassProhibitCustomPalettes {}

impl LintPass for LintPassProhibitCustomPalettes {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(kra_palette_container) = kra_archive.main_doc.image.palette_container.as_ref() {
                for kra_palette in kra_palette_container.into_iter() {
                    lint_messages.push(
                        "Prohibited use of custom palettes",
                        format!("Palette: \"{}\"", kra_palette.name.escape_debug()),
                    );
                }
            }
        }

        Ok(())
    }
}
