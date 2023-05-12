use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::{LintLayerProperty, LintMaskProperty};
use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassHiddenSurface {
    hidden_layers: LintLayerProperty<bool>,
    hidden_masks: LintMaskProperty<bool>,
}

impl LintPass for LintPassHiddenSurface {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                let (layer_opt, layer_display) = self.hidden_layers.get(layer)?;

                #[allow(clippy::collapsible_if)]
                if *layer_opt == Some(false) {
                    if (layer.visible == 0) || (layer.opacity == 0) {
                        lint_messages.push(format!("Prohibited hidden {} (layer: \"{}\")", layer_display, layer.name));
                    }
                }
            }
        }

        // Sub-pass #2
        {
            for (layer, mask) in kra_archive.all_masks() {
                let (mask_opt, mask_display) = self.hidden_masks.get(mask)?;

                #[allow(clippy::collapsible_if)]
                if *mask_opt == Some(false) {
                    // Bug: Interface allows setting opacity for some types of masks,
                    //   however they are not stored in the KRA documents.
                    if mask.visible == 0 {
                        lint_messages.push(format!(
                            "Prohibited hidden {} (layer: \"{}\", mask: \"{}\")",
                            mask_display, layer.name, mask.name
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}
