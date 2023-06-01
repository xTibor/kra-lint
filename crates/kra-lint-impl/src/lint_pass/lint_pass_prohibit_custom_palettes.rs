use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::meta_comment;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassProhibitCustomPalettes {}

impl LintPass for LintPassProhibitCustomPalettes {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(kra_palette_container) = kra_archive.main_doc.image.palette_container.as_ref() {
                for kra_palette in kra_palette_container.into_iter() {
                    // Bug: KRA files known to have mangled internal palettes (clusterfuck around bit depths/color spaces)
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Prohibited use of custom palettes",
                        &[
                            meta_comment!(format!("Palette: \"{}\"", kra_palette.name)),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
