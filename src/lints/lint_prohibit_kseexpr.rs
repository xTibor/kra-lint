use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassProhibitKSeExpr {}

impl LintPass for LintPassProhibitKSeExpr {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                if (layer.node_type == "generatorlayer")
                    && (layer.generator_name == Some("seexpr".to_owned()))
                {
                    results.push(format!(
                        "Prohibited use of KSeExpr (layer: \"{}\")",
                        layer.name
                    ));
                }
            }
        }

        results
    }
}
