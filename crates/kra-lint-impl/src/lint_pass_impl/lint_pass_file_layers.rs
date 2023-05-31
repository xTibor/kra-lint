use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::{KraLayerType, KraScalingMethod};

use crate::lint_fields::{LintGenericMatchExpression, LintStringMatchExpression};
use crate::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_comment, meta_expected, meta_found, meta_layer};

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
                                        meta_layer!(layer),
                                        meta_expected!(file_formats),
                                        meta_found!(source_ext),
                                    ],
                                );
                            }
                        } else {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "File layer source image has no file extension",
                                &[
                                    meta_layer!(layer),
                                    meta_expected!(file_formats),
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
                                    meta_layer!(layer),
                                    meta_comment!(format!("Source: \"{}\"", source)),
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
                                    meta_layer!(layer),
                                    meta_expected!(scaling_method),
                                    meta_found!(kra_scaling_method),
                                ],
                            );
                        }
                    } else {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Missing file layer scaling method",
                            &[
                                meta_layer!(layer),
                                meta_expected!(scaling_method),
                            ],
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
