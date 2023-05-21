use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::{LintLayerProperty, LintMaskProperty, LintNumberMatchExpression};
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{LintMessages, LintMetadata};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassAnimation {
    animated_layers: LintLayerProperty<bool>,
    animated_masks: LintMaskProperty<bool>,
    framerate: Option<LintNumberMatchExpression<usize>>,
}

impl LintPass for LintPassAnimation {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                let (layer_opt, layer_display) = self.animated_layers.get(layer);

                #[allow(clippy::collapsible_if)]
                if *layer_opt == Some(false) {
                    if layer.keyframes.is_some() {
                        #[rustfmt::skip]
                        lint_messages.push(
                            format!("Prohibited use of animated {}", layer_display),
                            &[
                                LintMetadata::Layer(layer.name.to_string(), layer.uuid.to_string()),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #2
        {
            for (layer, mask) in kra_archive.all_masks() {
                let (mask_opt, mask_display) = self.animated_masks.get(mask);

                #[allow(clippy::collapsible_if)]
                if *mask_opt == Some(false) {
                    if mask.keyframes.is_some() {
                        #[rustfmt::skip]
                        lint_messages.push(
                            format!("Prohibited use of animated {}", mask_display),
                            &[
                                LintMetadata::Layer(layer.name.to_string(), layer.uuid.to_string()),
                                LintMetadata::Mask(mask.name.to_string(), mask.uuid.to_string()),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #3
        {
            if let Some(framerate) = self.framerate.as_ref() {
                let kra_framerate = kra_archive.main_doc.image.animation.framerate.value;

                if !framerate.matches(&kra_framerate) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect animation framerate",
                        &[
                            LintMetadata::Expected(format!("{}fps", framerate)),
                            LintMetadata::Found(format!("{}fps", kra_framerate)),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
