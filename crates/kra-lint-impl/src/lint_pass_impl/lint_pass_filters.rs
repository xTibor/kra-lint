use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_filterconfig::KraPixelizeFilterConfig;
use kra_parser::kra_maindoc::{KraLayerType, KraMainDocLayer, KraMainDocMask, KraMaskType};

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_messages::{LintMessages, LintMetadata};
use crate::lint_pass::{LintPass, LintPassResult};

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
                                    LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                    LintMetadata::Expected(filter_types.to_string()),
                                    LintMetadata::Found(kra_filter_type.to_string()),
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
                                    LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                    LintMetadata::Mask { mask_name: mask.name.to_string(), mask_uuid: mask.uuid.to_string() },
                                    LintMetadata::Expected(filter_types.to_string()),
                                    LintMetadata::Found(kra_filter_type.to_string()),
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

                #[rustfmt::skip]
                let layer_mask_lint_metadata = |layer: &KraMainDocLayer, mask: Option<&KraMainDocMask>| {
                    if let Some(mask) = mask {
                        vec![
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                            LintMetadata::Mask { mask_name: mask.name.to_string(), mask_uuid: mask.uuid.to_string() },
                        ]
                    } else {
                        vec![
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                        ]
                    }
                };

                let lint_filter_config = |filter_config: KraPixelizeFilterConfig,
                                          layer: &KraMainDocLayer,
                                          mask: Option<&KraMainDocMask>,
                                          lint_messages: &mut LintMessages| {
                    if filter_config.pixel_width != filter_config.pixel_height {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect Pixiv mosaics",
                            &[
                                vec![
                                    LintMetadata::Comment("Non-square mosaics".to_owned()),
                                ],
                                layer_mask_lint_metadata(layer, mask),
                                vec![
                                    LintMetadata::Expected(format!("{}x{}px", minimum_mosaic_size, minimum_mosaic_size)),
                                    LintMetadata::Found(format!("{}x{}px", filter_config.pixel_width, filter_config.pixel_height)),
                                ],
                            ]
                            .concat(),
                        );
                    } else if filter_config.pixel_width < minimum_mosaic_size {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect Pixiv mosaics",
                            &[
                                vec![
                                    LintMetadata::Comment("Insufficient mosaic size".to_owned()),
                                ],
                                layer_mask_lint_metadata(layer, mask),
                                vec![
                                    LintMetadata::Expected(format!("{}x{}px", minimum_mosaic_size, minimum_mosaic_size)),
                                    LintMetadata::Found(format!("{}x{}px", filter_config.pixel_width, filter_config.pixel_height)),
                                ],
                            ]
                            .concat(),
                        );
                    }
                };

                for layer in kra_archive.all_layers_by_type(KraLayerType::FilterLayer) {
                    if layer.filter_name.as_deref() == Some("pixelize") {
                        let filter_config = layer.filter_config::<KraPixelizeFilterConfig>(kra_archive)?;
                        lint_filter_config(filter_config, layer, None, lint_messages);

                        if layer.opacity < 255 {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect Pixiv mosaics",
                                &[
                                    LintMetadata::Comment("Transparent mosaic filter layer".to_owned()),
                                    LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                ],
                            );
                        }
                    }
                }

                for (layer, mask) in kra_archive.all_masks_by_type(KraMaskType::FilterMask) {
                    if mask.filter_name.as_deref() == Some("pixelize") {
                        let filter_config = mask.filter_config::<KraPixelizeFilterConfig>(kra_archive)?;
                        lint_filter_config(filter_config, layer, Some(mask), lint_messages);
                    }
                }
            }
        }

        Ok(())
    }
}
