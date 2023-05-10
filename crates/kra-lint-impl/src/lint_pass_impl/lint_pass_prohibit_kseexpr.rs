use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassProhibitKSeExpr {}

impl LintPass for LintPassProhibitKSeExpr {
    fn lint(
        &self,
        kra_archive: &KraArchive,
        lint_messages: &mut Vec<String>,
    ) -> LintPassResult {
        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                if (layer.node_type == "generatorlayer")
                    && (layer.generator_name.as_deref() == Some("seexpr"))
                {
                    lint_messages.push(format!(
                        "Prohibited use of KSeExpr (layer: \"{}\")",
                        layer.name
                    ));
                }
            }
        }

        Ok(())
    }
}
