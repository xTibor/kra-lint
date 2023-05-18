use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::{LintMessages, LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassColorspace {
    colorspace: LintStringMatchExpression,
    profile: LintStringMatchExpression,
}

impl LintPass for LintPassColorspace {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_colorspace = &kra_archive.main_doc.image.colorspace_name;

            if !self.colorspace.matches(kra_colorspace) {
                lint_messages.push(
                    "Incorrect document color space",
                    format!("Expected: {}, Found: \"{}\"", self.colorspace, kra_colorspace.escape_debug()),
                );
            }
        }

        // Sub-pass #2
        {
            for layer in kra_archive.all_layers() {
                if let Some(layer_colorspace) = layer.colorspace_name.as_ref() {
                    if !self.colorspace.matches(layer_colorspace) {
                        lint_messages.push(
                            "Incorrect layer color space",
                            format!(
                                "Layer: \"{}\", Expected: {}, Found: \"{}\"",
                                layer.name.escape_debug(),
                                self.colorspace,
                                layer_colorspace.escape_debug()
                            ),
                        );
                    }
                }
            }
        }

        // Sub-pass #3
        {
            for (layer, mask) in kra_archive.all_masks() {
                if let Some(mask_colorspace) = mask.colorspace_name.as_ref() {
                    if !self.colorspace.matches(mask_colorspace) {
                        lint_messages.push(
                            "Incorrect mask color space",
                            format!(
                                "Layer: \"{}\", Mask: \"{}\", Expected: {}, Found: \"{}\"",
                                layer.name.escape_debug(),
                                mask.name.escape_debug(),
                                self.colorspace,
                                mask_colorspace.escape_debug()
                            ),
                        );
                    }
                }
            }
        }

        // Sub-pass #4
        {
            let kra_profile = &kra_archive.main_doc.image.profile;

            if !self.profile.matches(kra_profile) {
                lint_messages.push(
                    "Incorrect document color profile",
                    format!("Expected: {}, Found: \"{}\"", self.profile, kra_profile.escape_debug()),
                );
            }
        }

        // Sub-pass #5
        {
            // TODO: Lint layer color profiles
        }

        Ok(())
    }
}
