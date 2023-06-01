use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{KraLayerType, KraMaskType};

use crate::lint_output::macros::{meta_expected, meta_found, meta_layer, meta_mask};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassNonDefaultBlending {}

impl LintPass for LintPassNonDefaultBlending {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                if layer.opacity != 255 {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Non-default layer transparency",
                        &[
                            meta_layer!(layer),
                            meta_expected!(format!("{:.0}%", 100.0)),
                            meta_found!(format!("{:.0}%", layer.opacity as f64 / 255.0 * 100.0)),
                        ],
                    );
                }
            }
        }

        // Sub-pass #2
        {
            for layer in kra_archive.all_layers() {
                let expected_blending_mode = match layer.layer_type {
                    KraLayerType::FilterLayer => "copy",
                    _ => "normal",
                };

                if layer.composite_op != expected_blending_mode {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Non-default layer blending mode",
                        &[
                            meta_layer!(layer),
                            meta_expected!(expected_blending_mode),
                            meta_found!(layer.composite_op),
                        ],
                    );
                }
            }
        }

        // Sub-pass #3
        {
            for (layer, mask) in kra_archive.all_masks() {
                let expected_blending_mode = match mask.mask_type {
                    KraMaskType::ColorizeMask => Some("behind"),
                    _ => None,
                };

                if mask.composite_op.as_deref() != expected_blending_mode {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Non-default mask blending mode",
                        &[
                            meta_layer!(layer),
                            meta_mask!(mask),
                            meta_expected!(expected_blending_mode.unwrap_or("none")),
                            meta_found!(mask.composite_op.as_deref().unwrap_or("none")),
                        ],
                    );
                }
            }
        }

        // Sub-pass #4
        {
            for layer in kra_archive.all_layers() {
                if (layer.channel_flags != "1111") && !layer.channel_flags.is_empty() {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Non-default set of active channels",
                        &[
                            meta_layer!(layer),
                            meta_expected!("1111"),
                            meta_found!(layer.channel_flags),
                        ],
                    );
                }
            }
        }

        // Sub-pass #5
        {
            for layer in kra_archive.all_layers() {
                if let Some(channel_lock_flags) = layer.channel_lock_flags.as_ref() {
                    if (channel_lock_flags != "1111") && !channel_lock_flags.is_empty() {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Non-default channel lock flags",
                            &[
                                meta_layer!(layer),
                                meta_expected!("1111"),
                                meta_found!(channel_lock_flags),
                            ],
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
