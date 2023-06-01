use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::KraMaskType;

use crate::lint_output::lint_metadata_macros::{meta_layer, meta_mask};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassColorizeMask {
    warn_keystrokes_edit_mode: Option<bool>,
    enforce_coloring: Option<bool>,
}

impl LintPass for LintPassColorizeMask {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if self.warn_keystrokes_edit_mode == Some(true) {
                for (layer, mask) in kra_archive.all_masks_by_type(KraMaskType::ColorizeMask) {
                    if mask.edit_keystrokes == Some(true) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Active key stroke edit modes on colorize masks leading to false document previews",
                            &[
                                meta_layer!(layer),
                                meta_mask!(mask),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #2
        {
            if self.enforce_coloring == Some(true) {
                for (layer, mask) in kra_archive.all_masks_by_type(KraMaskType::ColorizeMask) {
                    if mask.show_coloring == Some(false) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Disabled coloring on colorize mask",
                            &[
                                meta_layer!(layer),
                                meta_mask!(mask),
                            ],
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
