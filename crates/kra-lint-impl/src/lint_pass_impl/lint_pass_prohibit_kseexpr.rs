use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_utils::KraLayerType;

use crate::{LintMessages, LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassProhibitKSeExpr {}

impl LintPass for LintPassProhibitKSeExpr {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                if (layer.layer_type()? == KraLayerType::FillLayer)
                    && (layer.generator_name.as_deref() == Some("seexpr"))
                {
                    lint_messages.push(format!("Prohibited use of KSeExpr (layer: \"{}\")", layer.name));
                }
            }
        }

        Ok(())
    }
}
