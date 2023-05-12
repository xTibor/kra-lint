use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintNumberMatchExpression;
use crate::{LintPass, LintPassResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LintPassDocumentSizeEntry {
    width: LintNumberMatchExpression<usize>,
    height: LintNumberMatchExpression<usize>,
    resolution: LintNumberMatchExpression<f64>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassDocumentSize {
    document_sizes: Vec<LintPassDocumentSizeEntry>,
}

impl LintPass for LintPassDocumentSize {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
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
                    .map(|ds| format!("{}×{}px/{}dpi)", ds.width, ds.height, ds.resolution))
                    .collect::<Vec<_>>()
                    .join(", ");

                lint_messages.push(format!(
                    "Incorrect document size (expected: [{}], found: {}×{}px/{}dpi)",
                    document_size_list, kra_document_width, kra_document_height, kra_document_resolution
                ));
            }
        }

        // Sub-pass #2
        {
            let kra_resolution_x = kra_archive.main_doc.image.x_res;
            let kra_resolution_y = kra_archive.main_doc.image.y_res;

            if kra_resolution_x != kra_resolution_y {
                lint_messages.push(format!(
                    "Inconsistent horizontal and vertical document resolution (horizontal: {}dpi, vertical: {}dpi)",
                    kra_resolution_x, kra_resolution_y,
                ));
            }
        }

        Ok(())
    }
}
