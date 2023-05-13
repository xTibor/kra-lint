use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintLayerProperty;
use crate::{LintMessages, LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassLayerStyles {
    styleable_layers: LintLayerProperty<bool>,
}

impl LintPass for LintPassLayerStyles {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                let (layer_opt, layer_display) = self.styleable_layers.get(layer)?;

                #[allow(clippy::collapsible_if)]
                if *layer_opt == Some(false) {
                    if layer.layer_style.is_some() {
                        // Bug: When removing all layer styles this KRA field does
                        //  not get cleared, interface still acts like layer styles
                        //  are present.
                        lint_messages.push(format!(
                            "Prohibited use of layer styles on {} (layer: \"{}\")",
                            layer_display, layer.name
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}
