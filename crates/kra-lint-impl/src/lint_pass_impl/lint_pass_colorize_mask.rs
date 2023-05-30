use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::KraMaskType;

use crate::lint_messages::{LintMessages, LintMetadata};
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassColorizeMask {
    warn_keystrokes_edit_mode: Option<bool>,
}

impl LintPass for LintPassColorizeMask {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if self.warn_keystrokes_edit_mode == Some(true) {
                for (layer, mask) in kra_archive.all_masks_by_type(KraMaskType::ColorizeMask) {
                    if mask.edit_keystrokes == Some(1) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Active key strokes edit mode on colorize mask",
                            &[
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                LintMetadata::Mask { mask_name: mask.name.to_string(), mask_uuid: mask.uuid.to_string() },
                                LintMetadata::Comment("Leads to false document previews".to_owned()),
                            ],
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
