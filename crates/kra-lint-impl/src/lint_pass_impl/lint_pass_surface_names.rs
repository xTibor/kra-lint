use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::{LintLayerProperty, LintMaskProperty, LintStringMatchExpression};
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{LintMessages, LintMetadata};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassSurfaceNames {
    layer_names: Option<LintLayerProperty<LintStringMatchExpression>>,
    mask_names: Option<LintMaskProperty<LintStringMatchExpression>>,
}

impl LintPass for LintPassSurfaceNames {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(layer_names) = self.layer_names.as_ref() {
                for layer in kra_archive.all_layers() {
                    let (layer_opt, layer_display) = layer_names.get(layer);

                    if let Some(string_match_expr) = layer_opt.as_ref() {
                        if !string_match_expr.matches(&layer.name) {
                            #[rustfmt::skip]
                            lint_messages.push(
                                format!("Incorrect {} name", layer_display),
                                &[
                                    LintMetadata::Layer(layer.name.to_string(), layer.uuid.to_string()),
                                    LintMetadata::Expected(string_match_expr.to_string()),
                                    LintMetadata::Found(layer.name.to_string()),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(mask_names) = self.mask_names.as_ref() {
                for (layer, mask) in kra_archive.all_masks() {
                    let (mask_opt, mask_display) = mask_names.get(mask);

                    if let Some(string_match_expr) = mask_opt.as_ref() {
                        if !string_match_expr.matches(&mask.name) {
                            #[rustfmt::skip]
                            lint_messages.push(
                                format!("Incorrect {} name", mask_display),
                                &[
                                    LintMetadata::Layer(layer.name.to_string(), layer.uuid.to_string()),
                                    LintMetadata::Mask(mask.name.to_string(), mask.uuid.to_string()),
                                    LintMetadata::Expected(string_match_expr.to_string()),
                                    LintMetadata::Found(mask.name.to_string()),
                                ],
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
