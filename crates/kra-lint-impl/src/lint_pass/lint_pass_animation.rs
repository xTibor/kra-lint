use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config_fields::{NumberMatchExpression, ValueByLayerType, ValueByMaskType};
use crate::lint_output::lint_metadata_macros::{meta_expected, meta_found, meta_layer, meta_mask};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassAnimation {
    animated_layers: Option<ValueByLayerType<bool>>,
    animated_masks: Option<ValueByMaskType<bool>>,
    framerate: Option<NumberMatchExpression<usize>>,
    force_layer_pin: Option<ValueByLayerType<bool>>,
    force_mask_pin: Option<ValueByMaskType<bool>>,
    warn_onion_skin: Option<bool>,
}

impl LintPass for LintPassAnimation {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(animated_layers) = self.animated_layers.as_ref() {
                for layer in kra_archive.all_layers() {
                    let (layer_opt, layer_display) = animated_layers.get(layer);

                    #[allow(clippy::collapsible_if)]
                    if *layer_opt == Some(false) {
                        if layer.keyframes.is_some() {
                            #[rustfmt::skip]
                            lint_messages.push(
                                format!("Prohibited use of animated {}", layer_display),
                                &[
                                    meta_layer!(layer),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(animated_masks) = self.animated_masks.as_ref() {
                for (layer, mask) in kra_archive.all_masks() {
                    let (mask_opt, mask_display) = animated_masks.get(mask);

                    #[allow(clippy::collapsible_if)]
                    if *mask_opt == Some(false) {
                        if mask.keyframes.is_some() {
                            #[rustfmt::skip]
                            lint_messages.push(
                                format!("Prohibited use of animated {}", mask_display),
                                &[
                                    meta_layer!(layer),
                                    meta_mask!(mask),
                                ],
                            );
                        }
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
                            meta_expected!(format!("{}fps", framerate)),
                            meta_found!(format!("{}fps", kra_framerate)),
                        ],
                    );
                }
            }
        }

        // Sub-pass #4
        {
            if let Some(force_layer_pin) = self.force_layer_pin.as_ref() {
                for layer in kra_archive.all_layers() {
                    let (layer_opt, layer_display) = force_layer_pin.get(layer);

                    #[allow(clippy::collapsible_if)]
                    if *layer_opt == Some(true) {
                        if layer.keyframes.is_some() && !layer.in_timeline {
                            #[rustfmt::skip]
                            lint_messages.push(
                                format!("Unpinned animated {}", layer_display),
                                &[
                                    meta_layer!(layer),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #5
        {
            if let Some(force_mask_pin) = self.force_mask_pin.as_ref() {
                for (layer, mask) in kra_archive.all_masks() {
                    let (mask_opt, mask_display) = force_mask_pin.get(mask);

                    #[allow(clippy::collapsible_if)]
                    if *mask_opt == Some(true) {
                        if mask.keyframes.is_some() && (mask.in_timeline != Some(true)) {
                            // Bug: Pinning/unpinning masks do not set the modified flag on the document.
                            //   Cannot save mask pinning changes by themselves without other unrelated changes.
                            #[rustfmt::skip]
                            lint_messages.push(
                                format!("Unpinned animated {}", mask_display),
                                &[
                                    meta_layer!(layer),
                                    meta_mask!(mask),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #6
        {
            for layer in kra_archive.all_layers() {
                if layer.onion_skin == Some(true) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Active onion skins leading to false document previews",
                        &[
                            meta_layer!(layer),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
