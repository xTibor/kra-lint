use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{
    LintLayerProperty, LintMaskProperty, LintPass, LintPassResult,
    LintStringMatchExpression,
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassSurfaceNames {
    pub layer_names: Option<LintLayerProperty<LintStringMatchExpression>>,
    pub mask_names: Option<LintMaskProperty<LintStringMatchExpression>>,
}

impl LintPass for LintPassSurfaceNames {
    fn lint(
        &self,
        kra_archive: &KraArchive,
        lint_messages: &mut Vec<String>,
    ) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(layer_names) = self.layer_names.as_ref() {
                for layer in kra_archive.all_layers() {
                    let (layer_opt, layer_display) = layer_names.get(layer)?;

                    if let Some(string_match_expr) = layer_opt.as_ref() {
                        if !string_match_expr.matches(&layer.name) {
                            lint_messages.push(format!(
                                "Incorrect {} name (layer: \"{}\", expected: {})",
                                layer_display, layer.name, string_match_expr,
                            ));
                        }
                    }
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(mask_names) = self.mask_names.as_ref() {
                for (layer, mask) in kra_archive.all_masks() {
                    let (mask_opt, mask_display) = mask_names.get(mask)?;

                    if let Some(string_match_expr) = mask_opt.as_ref() {
                        if !string_match_expr.matches(&mask.name) {
                            lint_messages.push(format!(
                                "Incorrect {} name (layer: \"{}\", mask: \"{}\", expected: {})",
                                mask_display, layer.name, mask.name, string_match_expr,
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
