use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config_fields::NumberMatchExpression;
use crate::lint_output::lint_metadata_macros::{meta_comment, meta_expected, meta_found};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct LintPassDocumentSizeEntry {
    width: NumberMatchExpression<usize>,
    height: NumberMatchExpression<usize>,
    resolution: NumberMatchExpression<f64>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassDocumentSize {
    document_sizes: Vec<LintPassDocumentSizeEntry>,
}

impl LintPass for LintPassDocumentSize {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_document_width = kra_archive.main_doc.image.width;
            let kra_document_height = kra_archive.main_doc.image.height;
            let kra_document_resolution = kra_archive.main_doc.image.x_res;

            let document_size_matches = self.document_sizes.iter().any(|ds| {
                ds.width.matches(&kra_document_width)
                    && ds.height.matches(&kra_document_height)
                    && ds.resolution.matches(&kra_document_resolution)
            });

            if !document_size_matches {
                let document_size_list = self
                    .document_sizes
                    .iter()
                    .map(|ds| format!("{}×{}px/{}dpi", ds.width, ds.height, ds.resolution))
                    .collect::<Vec<_>>()
                    .join(", ");

                #[rustfmt::skip]
                lint_messages.push(
                    "Incorrect document size",
                    &[
                        meta_expected!(document_size_list),
                        meta_found!(format!("{}×{}px/{}dpi", kra_document_width, kra_document_height, kra_document_resolution)),
                    ],
                );
            }
        }

        // Sub-pass #2
        {
            let kra_resolution_x = kra_archive.main_doc.image.x_res;
            let kra_resolution_y = kra_archive.main_doc.image.y_res;

            if kra_resolution_x != kra_resolution_y {
                #[rustfmt::skip]
                lint_messages.push(
                    "Inconsistent horizontal and vertical document resolution",
                    &[
                        meta_comment!(format!("Horizontal: {}dpi, Vertical: {}dpi", kra_resolution_x, kra_resolution_y)),
                    ],
                );
            }
        }

        Ok(())
    }
}
