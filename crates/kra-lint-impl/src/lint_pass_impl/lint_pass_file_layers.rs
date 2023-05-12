use camino::Utf8Path;
use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_utils::KraLayerType;

use crate::lint_fields::LintStringMatchExpression;
use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassFileLayers {
    file_formats: Option<LintStringMatchExpression>,
    check_missing_files: Option<bool>,
}

impl LintPass for LintPassFileLayers {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(file_formats) = self.file_formats.as_ref() {
                for layer in kra_archive.all_layers() {
                    if layer.layer_type()? == KraLayerType::FileLayer {
                        if let Some(source) = layer.source.as_ref() {
                            if let Some(source_ext) = Utf8Path::new(source).extension() {
                                if !file_formats.matches(source_ext) {
                                    lint_messages.push(format!(
                                        "Incorrect file layer source image format (layer: \"{}\", expected: {}, found: \"{}\")",
                                        layer.name, file_formats, source_ext,
                                    ));
                                }
                            } else {
                                lint_messages.push(format!(
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
            if self.check_missing_files == Some(true) {
                for layer in kra_archive.all_layers() {
                    if layer.layer_type()? == KraLayerType::FileLayer {
                        if let Some(source) = layer.source.as_ref() {
                            // File layers store relative paths, path traversal is intentional here.
                            let source_path =
                                kra_archive.zip_path.parent().expect("Failed to get parent directory").join(source);

                            if !source_path.is_file() {
                                lint_messages.push(format!(
                                    "Missing file layer source image (layer: \"{}\", source: \"{}\")",
                                    layer.name, source,
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
