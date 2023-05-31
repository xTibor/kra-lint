use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::{LintLayerProperty, LintMaskProperty};
use crate::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_layer, meta_mask};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassSurfaceType {
    layer_types: LintLayerProperty<bool>,
    mask_types: LintMaskProperty<bool>,
}

impl LintPass for LintPassSurfaceType {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                let (layer_opt, layer_display) = self.layer_types.get(layer);

                if *layer_opt == Some(false) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        format!("Prohibited use of {}", layer_display),
                        &[
                            meta_layer!(layer),
                        ],
                    );
                }
            }
        }

        // Sub-pass #2
        {
            for (layer, mask) in kra_archive.all_masks() {
                let (mask_opt, mask_display) = self.mask_types.get(mask);

                if *mask_opt == Some(false) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        format!("Prohibited use of {}", mask_display),
                        &[
                            meta_layer!(layer),
                            meta_mask!(mask),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
