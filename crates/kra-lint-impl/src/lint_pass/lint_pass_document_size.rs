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
    width: Option<NumberMatchExpression<usize>>,
    height: Option<NumberMatchExpression<usize>>,
    resolution: Option<NumberMatchExpression<f64>>,
}

impl LintPassDocumentSizeEntry {
    fn matches(&self, kra_document_width: usize, kra_document_height: usize, kra_document_resolution: f64) -> bool {
        // TODO: Option::is_none_or()
        self.width.as_ref().map_or(true, |m| m.matches(&kra_document_width))
            && self.height.as_ref().map_or(true, |m| m.matches(&kra_document_height))
            && self.resolution.as_ref().map_or(true, |m| m.matches(&kra_document_resolution))
    }

    fn message_fmt(&self) -> String {
        fn format_field<T: std::fmt::Display>(field: &Option<NumberMatchExpression<T>>) -> String {
            field.as_ref().map(NumberMatchExpression::to_string).unwrap_or("(any)".to_owned())
        }
        format!("{}×{}px/{}dpi", format_field(&self.width), format_field(&self.height), format_field(&self.resolution))
    }
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

            let document_size_matches = self
                .document_sizes
                .iter()
                .any(|size_entry| size_entry.matches(kra_document_width, kra_document_height, kra_document_resolution));

            if !document_size_matches {
                let document_size_list = self
                    .document_sizes
                    .iter()
                    .map(LintPassDocumentSizeEntry::message_fmt)
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
