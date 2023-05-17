use itertools::Itertools;
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{
    KraColorLabel, KraLayerType, KraMainDocLayer, KraMainDocLayerContainer, KraMainDocMask, KraMainDocMaskContainer,
    KraMaskType,
};

use crate::lint_fields::{LintGenericMatchExpression, LintNumberMatchExpression, LintStringMatchExpression};
use crate::{LintMessages, LintPass, LintPassResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize, Default)]
struct DocumentStructureMaskContainer(Vec<DocumentStructureMask>);

// TODO: #[derive(Iterator)]
impl DocumentStructureMaskContainer {
    pub fn iter(&self) -> impl Iterator<Item = &DocumentStructureMask> {
        self.0.iter()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
struct DocumentStructureMask {
    mask_name: Option<LintStringMatchExpression>,
    mask_type: Option<LintGenericMatchExpression<KraMaskType>>,
    mask_color: Option<LintGenericMatchExpression<KraColorLabel>>,
    mask_count: Option<LintNumberMatchExpression<usize>>,
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

#[derive(Debug, Deserialize, Serialize, Default)]
struct DocumentStructureLayerContainer(Vec<DocumentStructureLayer>);

// TODO: #[derive(Iterator)]
impl DocumentStructureLayerContainer {
    pub fn iter(&self) -> impl Iterator<Item = &DocumentStructureLayer> {
        self.0.iter()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
struct DocumentStructureLayer {
    layer_name: Option<LintStringMatchExpression>,
    layer_type: Option<LintGenericMatchExpression<KraLayerType>>,
    layer_color: Option<LintGenericMatchExpression<KraColorLabel>>,
    layer_count: Option<LintNumberMatchExpression<usize>>,
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
                let mut kra_mask_iterator = kra_mask_container.iter();

                for lint_mask in lint_mask_container.iter() {
                    let kra_matching_masks = kra_mask_iterator
                        .by_ref()
                        .peekable()
                        .peeking_take_while(|kra_mask| lint_mask.matches(kra_mask))
                        .collect::<Vec<_>>();

                    let mask_count = lint_mask.mask_count.as_ref().unwrap_or(&LintNumberMatchExpression::Value(1));

                    if !mask_count.matches(&kra_matching_masks.len()) {
                        lint_messages.push(format!(
                            "Incorrect document structure (Mask repetition mismatch, mask: ({}), expected: {}, found: {})",
                            lint_mask.message_fmt(), mask_count, kra_matching_masks.len(),
                        ));
                        return Ok(());
                    }
                }

                for kra_extra_mask in kra_mask_iterator {
                    lint_messages
                        .push(format!("Incorrect document structure (Extra mask, mask: \"{}\")", kra_extra_mask.name));
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

                let mut kra_layer_iterator = kra_layer_container.iter();

                for lint_layer in lint_layer_container.iter() {
                    let kra_matching_layers = kra_layer_iterator
                        .by_ref()
                        .peekable()
                        .peeking_take_while(|kra_layer| lint_layer.matches(kra_layer))
                        .collect::<Vec<_>>();

                    let layer_count = lint_layer.layer_count.as_ref().unwrap_or(&LintNumberMatchExpression::Value(1));

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
                        lint_messages.push(format!(
                            "Incorrect document structure (Layer repetition mismatch, layer: ({}), expected: {}, found: {})",
                            lint_layer.message_fmt(), layer_count, kra_matching_layers.len(),
                        ));
                        return Ok(());
                    }
                }

                for kra_extra_layer in kra_layer_iterator {
                    lint_messages.push(format!(
                        "Incorrect document structure (Extra layer, layer: \"{}\")",
                        kra_extra_layer.name
                    ));
                }

                Ok(())
            }

            compare_layers(&kra_archive.main_doc.image.layer_container, &self.layers, lint_messages)?;
        }

        Ok(())
    }
}
