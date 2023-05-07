use camino::Utf8Path;
use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult, StringMatchExpression};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassFileLayers {
    pub file_formats: Option<StringMatchExpression>,
    pub check_missing_files: Option<bool>,
}

impl LintPass for LintPassFileLayers {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            if let Some(file_formats) = self.file_formats.as_ref() {
                for layer in kra_archive.all_layers() {
                    if layer.node_type.as_str() == "filelayer" {
                        if let Some(source) = layer.source.as_ref() {
                            if let Some(source_ext) =
                                Utf8Path::new(source).extension()
                            {
                                if !file_formats.matches(source_ext) {
                                    results.push(format!(
                                        "Incorrect file layer source image format (layer: \"{}\", expected: {}, found: \"{}\")",
                                        layer.name, file_formats, source_ext,
                                    ));
                                }
                            } else {
                                results.push(format!(
                                    "File layer source image has no file extension (layer: \"{}\", expected: {})",
                                     layer.name, file_formats,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Sub-pass #2
        {
            // TODO: self.check_missing_files
        }

        results
    }
}
