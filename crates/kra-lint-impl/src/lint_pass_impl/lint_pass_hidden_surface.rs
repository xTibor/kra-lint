use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config_fields::{ValueByLayerType, ValueByMaskType};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_layer, meta_mask};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassHiddenSurface {
    hidden_layers: ValueByLayerType<bool>,
    hidden_masks: ValueByMaskType<bool>,
}

impl LintPass for LintPassHiddenSurface {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                let (layer_opt, layer_display) = self.hidden_layers.get(layer);

                #[allow(clippy::collapsible_if)]
                if *layer_opt == Some(false) {
                    if !layer.visible || (layer.opacity == 0) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            format!("Prohibited hidden {}", layer_display),
                            &[
                                meta_layer!(layer),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #2
        {
            for (layer, mask) in kra_archive.all_masks() {
                let (mask_opt, mask_display) = self.hidden_masks.get(mask);

                #[allow(clippy::collapsible_if)]
                if *mask_opt == Some(false) {
                    // Bug: Interface allows setting opacity for some types of masks,
                    //   however they are not stored in the KRA documents.
                    if !mask.visible {
                        #[rustfmt::skip]
                        lint_messages.push(
                            format!("Prohibited hidden {}", mask_display),
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
