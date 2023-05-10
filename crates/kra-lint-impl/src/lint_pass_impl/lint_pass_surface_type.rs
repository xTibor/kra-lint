use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintLayerProperty, LintMaskProperty, LintPass, LintPassResult};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassSurfaceType {
    pub layer_types: LintLayerProperty<bool>,
    pub mask_types: LintMaskProperty<bool>,
}

impl LintPass for LintPassSurfaceType {
    fn lint(
        &self,
        kra_archive: &KraArchive,
        lint_messages: &mut Vec<String>,
    ) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                let (layer_opt, layer_display) = self.layer_types.get(layer)?;

                if *layer_opt == Some(false) {
                    lint_messages.push(format!(
                        "Prohibited use of {} (layer: \"{}\")",
                        layer_display, layer.name
                    ));
                }
            }
        }

        // Sub-pass #2
        {
            for (layer, mask) in kra_archive.all_masks() {
                let (mask_opt, mask_display) = self.mask_types.get(mask)?;

                if *mask_opt == Some(false) {
                    lint_messages.push(format!(
                        "Prohibited use of {} (layer: \"{}\", mask: \"{}\")",
                        mask_display, layer.name, mask.name
                    ));
                }
            }
        }

        Ok(())
    }
}
