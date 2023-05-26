use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{KraLayerType, KraMaskType};

use crate::lint_messages::{LintMessages, LintMetadata};
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
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                            LintMetadata::Expected(format!("{:.0}%", 100.0)),
                            LintMetadata::Found(format!("{:.0}%", layer.opacity as f64 / 255.0 * 100.0)),
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
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                            LintMetadata::Expected(expected_blending_mode.to_string()),
                            LintMetadata::Found(layer.composite_op.to_string()),
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
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                            LintMetadata::Mask { mask_name: mask.name.to_string(), mask_uuid: mask.uuid.to_string() },
                            LintMetadata::Expected(expected_blending_mode.unwrap_or("none").to_string()),
                            LintMetadata::Found(mask.composite_op.as_deref().unwrap_or("none").to_string()),
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
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                            LintMetadata::Expected("1111".to_string()),
                            LintMetadata::Found(layer.channel_flags.to_string()),
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
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                LintMetadata::Expected("1111".to_string()),
                                LintMetadata::Found(channel_lock_flags.to_string()),
                            ],
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
