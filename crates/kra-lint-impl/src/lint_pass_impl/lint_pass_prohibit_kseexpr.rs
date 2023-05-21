use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::KraLayerType;

use crate::lint_pass::{LintPass, LintPassResult};
use crate::{LintMessages, LintMetadata};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassProhibitKSeExpr {}

impl LintPass for LintPassProhibitKSeExpr {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers_by_type(KraLayerType::FillLayer) {
                if layer.generator_name.as_deref() == Some("seexpr") {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Prohibited use of KSeExpr",
                        &[
                            LintMetadata::Layer(layer.name.to_string(), layer.uuid.to_string()),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
