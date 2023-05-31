use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{KraLayerType, KraMaskType};

use sha2::{Digest, Sha256};

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_comment, meta_expected, meta_found, meta_layer, meta_mask};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassColorspace {
    colorspace: Option<LintStringMatchExpression>,
    profile: Option<LintStringMatchExpression>,
    profile_checksum: Option<LintStringMatchExpression>,
}

impl LintPass for LintPassColorspace {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(colorspace) = self.colorspace.as_ref() {
                let kra_colorspace = &kra_archive.main_doc.image.colorspace_name;

                if !colorspace.matches(kra_colorspace) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect document color space",
                        &[
                            meta_expected!(colorspace),
                            meta_found!(kra_colorspace),
                        ],
                    );
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(colorspace) = self.colorspace.as_ref() {
                for layer in kra_archive.all_layers() {
                    if let Some(layer_colorspace) = layer.colorspace_name.as_ref() {
                        if !colorspace.matches(layer_colorspace) {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect layer color space",
                                &[
                                    meta_layer!(layer),
                                    meta_expected!(colorspace),
                                    meta_found!(layer_colorspace),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #3
        {
            if let Some(colorspace) = self.colorspace.as_ref() {
                for (layer, mask) in kra_archive.all_masks() {
                    if let Some(mask_colorspace) = mask.colorspace_name.as_ref() {
                        if !colorspace.matches(mask_colorspace) {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect mask color space",
                                &[
                                    meta_layer!(layer),
                                    meta_mask!(mask),
                                    meta_expected!(colorspace),
                                    meta_found!(mask_colorspace),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #4
        {
            if let Some(profile) = self.profile.as_ref() {
                let kra_profile = &kra_archive.main_doc.image.profile;

                if !profile.matches(kra_profile) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect document color profile",
                        &[
                            meta_expected!(profile),
                            meta_found!(kra_profile),
                        ],
                    );
                }
            }
        }

        // Sub-pass #5
        {
            if let Some(profile_checksum) = self.profile_checksum.as_ref() {
                for layer in kra_archive.all_layers_by_type(KraLayerType::PaintLayer) {
                    let layer_color_profile = layer.color_profile(kra_archive)?;
                    let layer_color_profile_checksum =
                        base16ct::lower::encode_string(&Sha256::digest(layer_color_profile));

                    if !profile_checksum.matches(&layer_color_profile_checksum) {
                        // Bug: Vector layers do not persist color profiles, defaulting to
                        //   the document color profile on reload. I cannot lint this data loss.
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect layer color profile",
                            &[
                                meta_layer!(layer),
                                meta_comment!("Profile checksum mismatch"),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #6
        {
            if let Some(profile_checksum) = self.profile_checksum.as_ref() {
                for (layer, mask) in kra_archive.all_masks_by_type(KraMaskType::ColorizeMask) {
                    let mask_color_profile = mask.colorize_color_profile(kra_archive)?;
                    let mask_color_profile_checksum =
                        base16ct::lower::encode_string(&Sha256::digest(mask_color_profile));

                    if !profile_checksum.matches(&mask_color_profile_checksum) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect colorize mask color profile",
                            &[
                                meta_layer!(layer),
                                meta_mask!(mask),
                                meta_comment!("Profile checksum mismatch"),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #6
        {
            if let Some(profile_checksum) = self.profile_checksum.as_ref() {
                let document_color_profile = kra_archive.main_doc.image.color_profile(kra_archive)?;
                let document_color_profile_checksum =
                    base16ct::lower::encode_string(&Sha256::digest(document_color_profile));

                if !profile_checksum.matches(&document_color_profile_checksum) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect document color profile",
                        &[
                            meta_comment!("Profile checksum mismatch"),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
