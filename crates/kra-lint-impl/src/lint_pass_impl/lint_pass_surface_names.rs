use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::{LintLayerProperty, LintMaskProperty, LintStringMatchExpression};
use crate::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_expected, meta_found, meta_layer, meta_mask};

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
                                    meta_layer!(layer),
                                    meta_expected!(string_match_expr),
                                    meta_found!(layer.name),
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
                                    meta_layer!(layer),
                                    meta_mask!(mask),
                                    meta_expected!(string_match_expr),
                                    meta_found!(mask.name),
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
