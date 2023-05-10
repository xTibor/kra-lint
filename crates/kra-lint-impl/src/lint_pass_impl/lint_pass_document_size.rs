use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LintPassDocumentSizeEntry {
    pub width: usize,
    pub height: usize,
    pub resolution: f64,
}

impl std::fmt::Display for LintPassDocumentSizeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}×{}px/{}dpi", self.width, self.height, self.resolution)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassDocumentSize {
    pub document_sizes: Vec<LintPassDocumentSizeEntry>,
}

impl LintPass for LintPassDocumentSize {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_document_size = LintPassDocumentSizeEntry {
                width: kra_archive.main_doc.image.width,
                height: kra_archive.main_doc.image.height,
                resolution: kra_archive.main_doc.image.x_res,
            };

            if !self.document_sizes.contains(&kra_document_size) {
                lint_messages.push(format!(
                    "Incorrect document size (expected: [{}], found: {})",
                    self.document_sizes.iter().map(LintPassDocumentSizeEntry::to_string).collect::<Vec<_>>().join(", "),
                    kra_document_size,
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
