use itertools::Itertools;
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{KraMainDocLayer, KraMainDocLayerContainer};
use kra_parser::kra_utils::KraLayerType;

use crate::lint_fields::{LintGenericMatchExpression, LintNumberMatchExpression, LintStringMatchExpression};
use crate::{LintMessages, LintPass, LintPassResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
struct DocumentStructureLayerContainer(Vec<DocumentStructureLayer>);

#[derive(Debug, Deserialize, Serialize)]
struct DocumentStructureLayer {
    layer_name: LintStringMatchExpression,
    layer_type: LintGenericMatchExpression<KraLayerType>,
    layer_count: Option<LintNumberMatchExpression<usize>>,
    layers: Option<DocumentStructureLayerContainer>,
}

impl DocumentStructureLayer {
    fn matches(&self, kra_layer: &KraMainDocLayer) -> bool {
        self.layer_name.matches(&kra_layer.name) && self.layer_type.matches(&kra_layer.layer_type().unwrap())
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
            fn compare_layers(
                kra_layer_container: &KraMainDocLayerContainer,
                lint_layer_container: &DocumentStructureLayerContainer,
                lint_messages: &mut LintMessages,
            ) -> LintPassResult {
                let mut kra_layer_iterator = kra_layer_container.layers.iter();

                for lint_layer in lint_layer_container.0.iter() {
                    let kra_matching_layers = kra_layer_iterator
                        .by_ref()
                        .peeking_take_while(|kra_layer| lint_layer.matches(kra_layer))
                        .collect::<Vec<_>>();

                    let layer_count = lint_layer.layer_count.as_ref().unwrap_or(&LintNumberMatchExpression::Value(1));

                    if layer_count.matches(&kra_matching_layers.len()) {
                        for kra_layer in kra_matching_layers {
                            if kra_layer.layer_type()? == KraLayerType::GroupLayer {
                                compare_layers(
                                    kra_layer.layer_container.as_ref().unwrap(),
                                    lint_layer.layers.as_ref().unwrap(),
                                    lint_messages,
                                )?;
                            }
                        }
                    } else {
                        lint_messages.push(format!(
                            "Incorrect document structure (Layer repetition mismatch, layer: {}, expected: {}, found: {})",
                            lint_layer.layer_name, layer_count, kra_matching_layers.len(),
                        ));
                        // Bail out after the first mismatch, otherwise this lint may generate false messages.
                        return Ok(());
                    }
                }

                Ok(())
            }

            compare_layers(&kra_archive.main_doc.image.layer_container, &self.layers, lint_messages)?;
        }

        Ok(())
    }
}
