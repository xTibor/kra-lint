use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{KraLayerType, KraMaskType};

use sha2::{Digest, Sha256};

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_messages::{LintMessages, LintMetadata};
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassColorspace {
    colorspace: LintStringMatchExpression,
    profile: LintStringMatchExpression,
    profile_checksum: Option<LintStringMatchExpression>,
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
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
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
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                LintMetadata::Mask { mask_name: mask.name.to_string(), mask_uuid: mask.uuid.to_string() },
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
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                LintMetadata::Comment("Profile checksum mismatch".to_owned()),
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
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                LintMetadata::Mask { mask_name: mask.name.to_string(), mask_uuid: mask.uuid.to_string() },
                                LintMetadata::Comment("Profile checksum mismatch".to_owned()),
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
                            LintMetadata::Comment("Profile checksum mismatch".to_owned()),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
