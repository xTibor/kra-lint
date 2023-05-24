use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{KraLayerType, KraMaskType};

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_messages::{LintMessages, LintMetadata};
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassFilters {
    filter_types: Option<LintStringMatchExpression>,
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

        Ok(())
    }
}
