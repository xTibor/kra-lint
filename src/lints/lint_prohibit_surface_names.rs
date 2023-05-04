use serde::Deserialize;

use crate::lints::{
    LintLayerTypeFlags, LintMaskTypeFlags, LintPass, LintPassResult,
};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassProhibitSurfaceNames {
    pub layer_names: Option<LintLayerTypeFlags<String>>,
    pub mask_names: Option<LintMaskTypeFlags<String>>,
}

impl LintPass for LintPassProhibitSurfaceNames {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            if let Some(layer_names) = self.layer_names.as_ref() {
                for layer in kra_archive.all_layers() {
                    let (layer_opt, layer_display) = layer_names.get(layer);

                    if let Some(layer_regex_str) = layer_opt.as_ref() {
                        let regex = regex::Regex::new(layer_regex_str)
                            .expect("Failed to compile regular expression");

                        if regex.is_match(&layer.name) {
                            results.push(format!(
                                "Prohibited {} name (layer: \"{}\")",
                                layer_display, layer.name
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
                    let (mask_opt, mask_display) = mask_names.get(mask);

                    if let Some(mask_regex_str) = mask_opt.as_ref() {
                        let regex = regex::Regex::new(mask_regex_str)
                            .expect("Failed to compile regular expression");

                        if regex.is_match(&mask.name) {
                            results.push(format!(
                                "Prohibited {} name (layer: \"{}\", mask: \"{}\")",
                                mask_display, layer.name, mask.name
                            ));
                        }
                    }
                }
            }
        }

        results
    }
}
