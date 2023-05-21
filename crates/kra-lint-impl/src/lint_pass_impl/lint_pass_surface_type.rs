use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::{LintLayerProperty, LintMaskProperty};
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{LintMessages, LintMetadata};

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
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
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
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                            LintMetadata::Mask { mask_name: mask.name.to_string(), mask_uuid: mask.uuid.to_string() },
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
