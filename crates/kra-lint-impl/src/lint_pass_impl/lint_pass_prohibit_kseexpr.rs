use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::KraLayerType;

use crate::lint_pass::{LintPass, LintPassResult};
use crate::LintMessages;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassProhibitKSeExpr {}

impl LintPass for LintPassProhibitKSeExpr {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers_by_type(KraLayerType::FillLayer) {
                if layer.generator_name.as_deref() == Some("seexpr") {
                    lint_messages
                        .push("Prohibited use of KSeExpr", format!("Layer: \"{}\"", layer.name.escape_debug()));
                }
            }
        }

        Ok(())
    }
}
