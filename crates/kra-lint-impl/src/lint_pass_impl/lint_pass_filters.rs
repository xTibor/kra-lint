use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_filterconfig::KraPixelizeFilterConfig;
use kra_parser::kra_maindoc::{KraLayerType, KraMaskType};

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_output::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_comment, meta_expected, meta_found, meta_layer, meta_mask};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassFilters {
    filter_types: Option<LintStringMatchExpression>,
    pixiv_mosaics: Option<bool>,
}

impl LintPass for LintPassFilters {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(filter_types) = self.filter_types.as_ref() {
                for layer in kra_archive.all_layers_by_type(KraLayerType::FilterLayer) {
                    if let Some(kra_filter_type) = layer.filter_name.as_ref() {
                        if !filter_types.matches(kra_filter_type) {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect filter layer type",
                                &[
                                    meta_layer!(layer),
                                    meta_expected!(filter_types),
                                    meta_found!(kra_filter_type),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(filter_types) = self.filter_types.as_ref() {
                for (layer, mask) in kra_archive.all_masks_by_type(KraMaskType::FilterMask) {
                    if let Some(kra_filter_type) = mask.filter_name.as_ref() {
                        if !filter_types.matches(kra_filter_type) {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect filter mask type",
                                &[
                                    meta_layer!(layer),
                                    meta_mask!(mask),
                                    meta_expected!(filter_types),
                                    meta_found!(kra_filter_type),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #3
        {
            if self.pixiv_mosaics == Some(true) {
                let minimum_mosaic_size = {
                    let kra_document_width = kra_archive.main_doc.image.width;
                    let kra_document_height = kra_archive.main_doc.image.height;
                    (kra_document_width.max(kra_document_height) / 100).max(4)
                };

                for layer in kra_archive.all_layers_by_type(KraLayerType::FilterLayer) {
                    if layer.filter_name.as_deref() == Some("pixelize") {
                        let filter_config = layer.filter_config::<KraPixelizeFilterConfig>(kra_archive)?;

                        if filter_config.pixel_width != filter_config.pixel_height {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect Pixiv mosaics",
                                &[
                                    meta_comment!("Non-square filter layer mosaics"),
                                    meta_layer!(layer),
                                    meta_expected!(format!("{}x{}px", minimum_mosaic_size, minimum_mosaic_size)),
                                    meta_found!(format!("{}x{}px", filter_config.pixel_width, filter_config.pixel_height)),
                                ],
                            );
                        } else if filter_config.pixel_width < minimum_mosaic_size {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect Pixiv mosaics",
                                &[
                                    meta_comment!("Insufficient filter layer mosaic size"),
                                    meta_layer!(layer),
                                    meta_expected!(format!("{}x{}px", minimum_mosaic_size, minimum_mosaic_size)),
                                    meta_found!(format!("{}x{}px", filter_config.pixel_width, filter_config.pixel_height)),
                                ],
                            );
                        }

                        if layer.opacity < 255 {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect Pixiv mosaics",
                                &[
                                    meta_comment!("Transparent mosaic filter layer"),
                                    meta_layer!(layer),
                                ],
                            );
                        }
                    }
                }

                for (layer, mask) in kra_archive.all_masks_by_type(KraMaskType::FilterMask) {
                    if mask.filter_name.as_deref() == Some("pixelize") {
                        let filter_config = mask.filter_config::<KraPixelizeFilterConfig>(kra_archive)?;

                        if filter_config.pixel_width != filter_config.pixel_height {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect Pixiv mosaics",
                                &[
                                    meta_comment!("Non-square filter mask mosaics"),
                                    meta_layer!(layer),
                                    meta_mask!(mask),
                                    meta_expected!(format!("{}x{}px", minimum_mosaic_size, minimum_mosaic_size)),
                                    meta_found!(format!("{}x{}px", filter_config.pixel_width, filter_config.pixel_height)),
                                ],
                            );
                        } else if filter_config.pixel_width < minimum_mosaic_size {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect Pixiv mosaics",
                                &[
                                    meta_comment!("Insufficient filter mask mosaic size"),
                                    meta_layer!(layer),
                                    meta_mask!(mask),
                                    meta_expected!(format!("{}x{}px", minimum_mosaic_size, minimum_mosaic_size)),
                                    meta_found!(format!("{}x{}px", filter_config.pixel_width, filter_config.pixel_height)),
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
