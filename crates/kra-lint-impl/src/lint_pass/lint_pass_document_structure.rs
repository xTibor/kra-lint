use derive_more::IntoIterator;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{
    KraColorLabel, KraLayerType, KraMainDocLayer, KraMainDocLayerContainer, KraMainDocMask, KraMainDocMaskContainer,
    KraMaskType,
};

use crate::lint_config_fields::{GenericMatchExpression, NumberMatchExpression, StringMatchExpression};
use crate::lint_output::lint_metadata_macros::{meta_comment, meta_expected, meta_found, meta_layer, meta_mask};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize, Default, IntoIterator)]
struct DocumentStructureMaskContainer (
    #[into_iterator(ref)]
    Vec<DocumentStructureMask>,
);

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
struct DocumentStructureMask {
    mask_name: Option<StringMatchExpression>,
    mask_type: Option<GenericMatchExpression<KraMaskType>>,
    mask_color: Option<GenericMatchExpression<KraColorLabel>>,
    mask_count: Option<NumberMatchExpression<usize>>,
}

impl DocumentStructureMask {
    #[rustfmt::skip]
    fn matches(&self, kra_mask: &KraMainDocMask) -> bool {
        // TODO: Option::is_none_or()
        self.mask_name.as_ref().map_or(true, |m| m.matches(&kra_mask.name))
            && self.mask_type.as_ref().map_or(true, |m| m.matches(&kra_mask.mask_type))
            && self.mask_color.as_ref().map_or(true, |m| {
                kra_mask.color_label.as_ref().is_some_and(|color_label| m.matches(color_label))
            })
    }

    fn message_fmt(&self) -> String {
        let mask_fields = [
            self.mask_name.as_ref().map(|mask_name| format!("mask name: {}", mask_name)),
            self.mask_type.as_ref().map(|mask_type| format!("mask type: {}", mask_type)),
            self.mask_color.as_ref().map(|mask_color| format!("mask color: {}", mask_color)),
        ];
        mask_fields.iter().flatten().join(", ")
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize, Default, IntoIterator)]
struct DocumentStructureLayerContainer (
    #[into_iterator(ref)]
    Vec<DocumentStructureLayer>,
);

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
struct DocumentStructureLayer {
    layer_name: Option<StringMatchExpression>,
    layer_type: Option<GenericMatchExpression<KraLayerType>>,
    layer_color: Option<GenericMatchExpression<KraColorLabel>>,
    layer_count: Option<NumberMatchExpression<usize>>,
    layers: Option<DocumentStructureLayerContainer>,
    masks: Option<DocumentStructureMaskContainer>,
}

impl DocumentStructureLayer {
    fn matches(&self, kra_layer: &KraMainDocLayer) -> bool {
        // TODO: Option::is_none_or()
        self.layer_name.as_ref().map_or(true, |m| m.matches(&kra_layer.name))
            && self.layer_type.as_ref().map_or(true, |m| m.matches(&kra_layer.layer_type))
            && self.layer_color.as_ref().map_or(true, |m| m.matches(&kra_layer.color_label))
    }

    fn message_fmt(&self) -> String {
        let layer_fields = [
            self.layer_name.as_ref().map(|layer_name| format!("layer name: {}", layer_name)),
            self.layer_type.as_ref().map(|layer_type| format!("layer type: {}", layer_type)),
            self.layer_color.as_ref().map(|layer_color| format!("layer color: {}", layer_color)),
        ];
        layer_fields.iter().flatten().join(", ")
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassDocumentStructure {
    layers: DocumentStructureLayerContainer,
}

impl LintPass for LintPassDocumentStructure {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            fn compare_masks(
                kra_mask_container: &KraMainDocMaskContainer,
                lint_mask_container: &DocumentStructureMaskContainer,
                lint_messages: &mut LintMessages,
            ) -> LintPassResult {
                let mut kra_mask_iterator = kra_mask_container.into_iter();

                for lint_mask in lint_mask_container.into_iter() {
                    let kra_matching_masks = kra_mask_iterator
                        .by_ref()
                        .peeking_take_while(|kra_mask| lint_mask.matches(kra_mask))
                        .collect::<Vec<_>>();

                    let mask_count = lint_mask.mask_count.as_ref().unwrap_or(&NumberMatchExpression::Value(1));

                    if !mask_count.matches(&kra_matching_masks.len()) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect document structure",
                            &[
                                meta_comment!("Mask repetition mismatch"),
                                meta_comment!(format!("Mask template: ({})", lint_mask.message_fmt())),
                                meta_expected!(mask_count),
                                meta_found!(kra_matching_masks.len()),
                            ],
                        );
                        return Ok(());
                    }
                }

                for kra_extra_mask in kra_mask_iterator {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect document structure",
                        &[
                            meta_comment!("Extra mask"),
                            meta_mask!(kra_extra_mask),
                        ],
                    );
                }

                Ok(())
            }

            fn compare_layers(
                kra_layer_container: &KraMainDocLayerContainer,
                lint_layer_container: &DocumentStructureLayerContainer,
                lint_messages: &mut LintMessages,
            ) -> LintPassResult {
                let dummy_kra_layer_container = KraMainDocLayerContainer::default();
                let dummy_kra_mask_container = KraMainDocMaskContainer::default();

                let dummy_lint_layer_container = DocumentStructureLayerContainer::default();
                let dummy_lint_mask_container = DocumentStructureMaskContainer::default();

                let mut kra_layer_iterator = kra_layer_container.into_iter();

                for lint_layer in lint_layer_container.into_iter() {
                    let kra_matching_layers = kra_layer_iterator
                        .by_ref()
                        .peeking_take_while(|kra_layer| lint_layer.matches(kra_layer))
                        .collect::<Vec<_>>();

                    let layer_count = lint_layer.layer_count.as_ref().unwrap_or(&NumberMatchExpression::Value(1));

                    if layer_count.matches(&kra_matching_layers.len()) {
                        for kra_layer in kra_matching_layers {
                            {
                                let lint_children_container =
                                    lint_layer.masks.as_ref().unwrap_or(&dummy_lint_mask_container);

                                let kra_children_container =
                                    kra_layer.mask_container.as_ref().unwrap_or(&dummy_kra_mask_container);

                                compare_masks(kra_children_container, lint_children_container, lint_messages)?;
                            }

                            if kra_layer.layer_type == KraLayerType::GroupLayer {
                                let lint_children_container =
                                    lint_layer.layers.as_ref().unwrap_or(&dummy_lint_layer_container);

                                let kra_children_container =
                                    kra_layer.layer_container.as_ref().unwrap_or(&dummy_kra_layer_container);

                                compare_layers(kra_children_container, lint_children_container, lint_messages)?;
                            }
                        }
                    } else {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect document structure",
                            &[
                                meta_comment!("Layer repetition mismatch"),
                                meta_comment!(format!("Layer template: ({})", lint_layer.message_fmt())),
                                meta_expected!(layer_count),
                                meta_found!(kra_matching_layers.len()),
                            ],
                        );
                        return Ok(());
                    }
                }

                for kra_extra_layer in kra_layer_iterator {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect document structure",
                        &[
                            meta_comment!("Extra layer"),
                            meta_layer!(kra_extra_layer),
                        ],
                    );
                }

                Ok(())
            }

            compare_layers(&kra_archive.main_doc.image.layer_container, &self.layers, lint_messages)?;
        }

        Ok(())
    }
}
