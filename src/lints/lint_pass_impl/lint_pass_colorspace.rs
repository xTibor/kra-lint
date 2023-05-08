use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult, LintStringMatchExpression};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassColorspace {
    pub colorspace: LintStringMatchExpression,
    pub profile: LintStringMatchExpression,
}

impl LintPass for LintPassColorspace {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            let kra_colorspace = &kra_archive.main_doc.image.colorspace_name;

            if !self.colorspace.matches(kra_colorspace) {
                results.push(format!(
                    "Incorrect document color space (expected: {}, found: \"{}\")",
                    self.colorspace, kra_colorspace
                ));
            }
        }

        // Sub-pass #2
        {
            for layer in kra_archive.all_layers() {
                if let Some(layer_colorspace) = layer.colorspace_name.as_ref() {
                    if !self.colorspace.matches(layer_colorspace) {
                        results.push(format!(
                            "Incorrect layer color space (layer: \"{}\", expected: {}, found: \"{}\")",
                            layer.name, self.colorspace, layer_colorspace
                        ));
                    }
                }
            }
        }

        // Sub-pass #3
        {
            for (layer, mask) in kra_archive.all_masks() {
                if let Some(mask_colorspace) = mask.colorspace_name.as_ref() {
                    if !self.colorspace.matches(mask_colorspace) {
                        results.push(format!(
                            "Incorrect mask color space (layer: \"{}\", mask: \"{}\", expected: {}, found: \"{}\")",
                            layer.name, mask.name, self.colorspace, mask_colorspace
                        ));
                    }
                }
            }
        }

        // Sub-pass #4
        {
            let kra_profile = &kra_archive.main_doc.image.profile;

            if !self.profile.matches(kra_profile) {
                results.push(format!(
                    "Incorrect document color profile (expected: {}, found: \"{}\")",
                    self.profile, kra_profile
                ));
            }
        }

        // Sub-pass #5
        {
            // TODO: Lint layer color profiles
        }

        results
    }
}
