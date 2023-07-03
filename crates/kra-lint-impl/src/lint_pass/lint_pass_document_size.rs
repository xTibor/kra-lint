use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use std_ext::OptionExt;

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
    aspect_ratio: Option<NumberMatchExpression<f64>>,
    resolution: Option<NumberMatchExpression<f64>>,
    rotation: Option<bool>,
}

impl LintPassDocumentSizeEntry {
    fn matches(&self, kra_width: usize, kra_height: usize, kra_resolution: f64) -> bool {
        let matches_inner = |kra_width: usize, kra_height: usize, kra_resolution: f64| -> bool {
            self.width.as_ref().is_none_or(|m| m.matches(&kra_width))
                && self.height.as_ref().is_none_or(|m| m.matches(&kra_height))
                && self.aspect_ratio.as_ref().is_none_or(|m| m.matches(&(kra_width as f64 / kra_height as f64)))
                && self.resolution.as_ref().is_none_or(|m| m.matches(&kra_resolution))
        };

        if self.rotation == Some(true) {
            matches_inner(kra_width, kra_height, kra_resolution) || matches_inner(kra_height, kra_width, kra_resolution)
        } else {
            matches_inner(kra_width, kra_height, kra_resolution)
        }
    }

    fn message_fmt(&self) -> String {
        fn format_field<T: std::fmt::Display>(field: &Option<NumberMatchExpression<T>>) -> String {
            field.as_ref().map(NumberMatchExpression::to_string).unwrap_or("(any)".to_owned())
        }

        fn format_aspect_ratio(aspect_ratio: &Option<NumberMatchExpression<f64>>) -> String {
            if let Some(aspect_ratio) = aspect_ratio {
                format!(" (aspect ratio: {})", aspect_ratio)
            } else {
                "".to_owned()
            }
        }

        format!(
            "{}×{}px/{}dpi{}{}",
            format_field(&self.width),
            format_field(&self.height),
            format_field(&self.resolution),
            format_aspect_ratio(&self.aspect_ratio),
            if self.rotation == Some(true) { " (rotatable)" } else { "" }
        )
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
            let kra_width = kra_archive.main_doc.image.width;
            let kra_height = kra_archive.main_doc.image.height;
            let kra_resolution = kra_archive.main_doc.image.x_res;

            let document_size_matches =
                self.document_sizes.iter().any(|size_entry| size_entry.matches(kra_width, kra_height, kra_resolution));

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
                        meta_found!(format!("{}×{}px/{}dpi", kra_width, kra_height, kra_resolution)),
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
