use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{LintMessages, LintMetadata};

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
                #[rustfmt::skip]
                lint_messages.push(
                    "Incorrect document color space",
                    &[
                        LintMetadata::Expected(self.colorspace.to_string()),
                        LintMetadata::Found(kra_colorspace.to_string()),
                    ],
                );
            }
        }

        // Sub-pass #2
        {
            for layer in kra_archive.all_layers() {
                if let Some(layer_colorspace) = layer.colorspace_name.as_ref() {
                    if !self.colorspace.matches(layer_colorspace) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect layer color space",
                            &[
                                LintMetadata::Layer(layer.name.to_string()),
                                LintMetadata::Expected(self.colorspace.to_string()),
                                LintMetadata::Found(layer_colorspace.to_string()),
                            ],
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
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect mask color space",
                            &[
                                LintMetadata::Layer(layer.name.to_string()),
                                LintMetadata::Mask(mask.name.to_string()),
                                LintMetadata::Expected(self.colorspace.to_string()),
                                LintMetadata::Found(mask_colorspace.to_string()),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #4
        {
            let kra_profile = &kra_archive.main_doc.image.profile;

            if !self.profile.matches(kra_profile) {
                #[rustfmt::skip]
                lint_messages.push(
                    "Incorrect document color profile",
                    &[
                        LintMetadata::Expected(self.profile.to_string()),
                        LintMetadata::Found(kra_profile.to_string()),
                    ],
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
