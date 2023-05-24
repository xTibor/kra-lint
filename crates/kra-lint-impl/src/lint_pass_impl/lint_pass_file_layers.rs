use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{KraLayerType, KraScalingMethod};

use crate::lint_fields::{LintGenericMatchExpression, LintStringMatchExpression};
use crate::lint_messages::{LintMessages, LintMetadata};
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassFileLayers {
    file_formats: Option<LintStringMatchExpression>,
    check_missing_files: Option<bool>,
    scaling_method: Option<LintGenericMatchExpression<KraScalingMethod>>,
}

impl LintPass for LintPassFileLayers {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(file_formats) = self.file_formats.as_ref() {
                for layer in kra_archive.all_layers_by_type(KraLayerType::FileLayer) {
                    if let Some(source) = layer.source.as_ref() {
                        if let Some(source_ext) = Utf8Path::new(source).extension() {
                            if !file_formats.matches(source_ext) {
                                #[rustfmt::skip]
                                lint_messages.push(
                                    "Incorrect file layer source image format",
                                    &[
                                        LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                        LintMetadata::Expected(file_formats.to_string()),
                                        LintMetadata::Found(source_ext.to_string()),
                                    ],
                                );
                            }
                        } else {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "File layer source image has no file extension",
                                &[
                                    LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                    LintMetadata::Expected(file_formats.to_string()),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #2
        {
            if self.check_missing_files == Some(true) {
                for layer in kra_archive.all_layers_by_type(KraLayerType::FileLayer) {
                    if let Some(source) = layer.source.as_ref() {
                        // File layers store relative paths, path traversal is intentional here.
                        let source_path =
                            kra_archive.zip_path.parent().expect("Failed to get parent directory").join(source);

                        if !source_path.is_file() {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Missing file layer source image",
                                &[
                                    LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                    LintMetadata::Comment(format!("Source: \"{}\"", source)),
                                ],
                            );
                        }
                    }
                }
            }
        }

        // Sub-pass #3
        {
            if let Some(scaling_method) = self.scaling_method.as_ref() {
                for layer in kra_archive.all_layers_by_type(KraLayerType::FileLayer) {
                    if let Some(kra_scaling_method) = layer.scaling_method.as_ref() {
                        if !scaling_method.matches(kra_scaling_method) {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect file layer scaling method",
                                &[
                                    LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                    LintMetadata::Expected(scaling_method.to_string()),
                                    LintMetadata::Found(kra_scaling_method.to_string()),
                                ],
                            );
                        }
                    } else {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Missing file layer scaling method",
                            &[
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                LintMetadata::Expected(scaling_method.to_string()),
                            ],
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
