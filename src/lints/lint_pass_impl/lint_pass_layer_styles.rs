use serde::Deserialize;

use crate::lints::{LintLayerProperty, LintPass, LintPassResult};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassLayerStyles {
    pub styleable_layers: LintLayerProperty<bool>,
}

impl LintPass for LintPassLayerStyles {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                let (layer_opt, layer_display) =
                    self.styleable_layers.get(layer);

                #[allow(clippy::collapsible_if)]
                if *layer_opt == Some(false) {
                    if layer.layer_style.is_some() {
                        // Bug: When removing all layer styles this KRA field does
                        //  not get cleared, interface still acts like layer styles
                        //  are present.
                        results.push(format!(
                            "Prohibited use of layer styles on {} (layer: \"{}\")",
                            layer_display, layer.name
                        ));
                    }
                }
            }
        }

        results
    }
}
