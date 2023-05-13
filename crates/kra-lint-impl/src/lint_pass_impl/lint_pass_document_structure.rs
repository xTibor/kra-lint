use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_utils::KraLayerType;

use crate::lint_fields::{LintGenericMatchExpression, LintStringMatchExpression};
use crate::{LintMessages, LintPass, LintPassResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
struct DocumentStructureLayerContainer(Vec<DocumentStructureLayer>);

#[derive(Debug, Deserialize, Serialize)]
struct DocumentStructureLayer {
    layer_name: Option<LintStringMatchExpression>,
    layer_type: Option<LintGenericMatchExpression<KraLayerType>>,
    layers: Option<DocumentStructureLayerContainer>,
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
            println!("{:#?}", self);
        }

        Ok(())
    }
}
