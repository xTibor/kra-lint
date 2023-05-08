use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassNonDefaultBlending {}

impl LintPass for LintPassNonDefaultBlending {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            for layer in kra_archive.all_layers() {
                if layer.opacity != 255 {
                    results.push(format!(
                        "Non-default layer transparency (layer: \"{}\", expected: \"{:.0}%\", found: \"{:.0}%\")",
                        layer.name, 100.0, (layer.opacity as f64 / 255.0 * 100.0),
                    ));
                }
            }
        }

        // Sub-pass #2
        {
            for layer in kra_archive.all_layers() {
                let expected_blending_mode = match layer.node_type.as_str() {
                    "adjustmentlayer" => "copy",
                    _ => "normal",
                };

                if layer.composite_op != expected_blending_mode {
                    results.push(format!(
                        "Non-default layer blending mode (layer: \"{}\", expected: \"{}\", found: \"{}\")",
                        layer.name, expected_blending_mode, layer.composite_op,
                    ));
                }
            }
        }

        // Sub-pass #3
        {
            for (layer, mask) in kra_archive.all_masks() {
                let expected_blending_mode = match mask.node_type.as_str() {
                    "colorizemask" => Some("behind"),
                    _ => None,
                };

                if mask.composite_op.as_deref() != expected_blending_mode {
                    results.push(format!(
                        "Non-default mask blending mode (layer: \"{}\", mask: \"{}\", expected: \"{}\", found: \"{}\")",
                        layer.name,
                        mask.name,
                        expected_blending_mode.unwrap_or("none"),
                        mask.composite_op.as_deref().unwrap_or("none"),
                    ));
                }
            }
        }

        results
    }
}