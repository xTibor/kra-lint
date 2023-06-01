use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_error::KraError;
use kra_parser::kra_maindoc::KraLayerType;

use ziparchive_ext::ZipArchiveExt;

use crate::lint_output::macros::{meta_bug, meta_comment, meta_layer};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassMalformedDocument {}

impl LintPass for LintPassMalformedDocument {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            let mut zip_archive = kra_archive.zip_archive.borrow_mut();

            if let Some(documentinfo_xml) = zip_archive.read_to_string("documentinfo.xml")? {
                if documentinfo_xml.matches("]]>").count() > 1 {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            meta_comment!("Unescaped documentinfo.xml <abstract> tag"),
                            meta_bug!(446376),
                        ],
                    );
                }
            } else {
                #[rustfmt::skip]
                lint_messages.push(
                    "Malformed document",
                    &[
                        meta_comment!("Missing documentinfo.xml"),
                    ],
                );
            }
        }

        // Sub-pass #2
        {
            let zip_archive = kra_archive.zip_archive.borrow();

            if zip_archive.file_names().any(|file_name| file_name.contains("../")) {
                #[rustfmt::skip]
                lint_messages.push(
                    "Malformed document",
                    &[
                        meta_comment!("Path traversal vulnerability"),
                        meta_bug!(429925),
                    ],
                );
            }
        }

        // Sub-pass #3
        {
            for layer in kra_archive.all_layers_by_type(KraLayerType::CloneLayer) {
                if let Some(clone_from_uuid) = layer.clone_from_uuid.as_ref() {
                    if !kra_archive.all_layers().any(|target_layer| &target_layer.uuid == clone_from_uuid) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Malformed document",
                            &[
                                meta_comment!("Missing clone layer target layer"),
                                meta_layer!(layer),
                                meta_bug!(414699),
                            ],
                        );
                    }
                } else {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            meta_comment!("Missing clone layer target field"),
                            meta_layer!(layer),
                        ],
                    );
                }
            }
        }

        // Sub-pass #4
        {
            if let Some(composition_container) = kra_archive.main_doc.image.composition_container.as_ref() {
                if composition_container.into_iter().any(|composition| composition.name.contains('/')) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            meta_comment!("Compositions path traversal vulnerability"),
                        ],
                    );
                }
            }
        }

        // Sub-pass #5
        {
            for layer in kra_archive.all_layers_by_type(KraLayerType::CloneLayer) {
                let uuid_root = &layer.uuid;

                let mut uuid_todo: Vec<&String> = vec![uuid_root];
                let mut uuid_done: Vec<&String> = vec![];

                while !uuid_todo.is_empty() {
                    let current_uuid = uuid_todo.pop().unwrap();
                    uuid_done.push(current_uuid);

                    let referencing_uuid = kra_archive
                        .all_layers()
                        .filter(|from_layer| from_layer.clone_from_uuid.as_ref() == Some(current_uuid))
                        .map(|from_layer| &from_layer.uuid)
                        .collect::<Vec<_>>();

                    if referencing_uuid.contains(&uuid_root) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Malformed document",
                            &[
                                meta_comment!("Clone layer loop"),
                                meta_layer!(layer),
                            ],
                        );
                        break;
                    }

                    let new_todo = referencing_uuid
                        .iter()
                        .filter(|from_uuid| !uuid_done.contains(from_uuid))
                        .filter(|from_uuid| !uuid_todo.contains(from_uuid))
                        .collect::<Vec<_>>();

                    uuid_todo.extend(new_todo);
                }
            }
        }

        // Sub-pass #6
        {
            let mut zip_archive = kra_archive.zip_archive.borrow_mut();

            let contains_mergedimage_png = zip_archive.exists("mergedimage.png")?;
            let file_extension = kra_archive.zip_path.extension().map(str::to_lowercase);

            match (file_extension.as_deref(), contains_mergedimage_png) {
                (Some("kra"), false) => {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            meta_comment!("KRA archive without required preview image"),
                        ],
                    );
                }
                (Some("krz"), true) => {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            meta_comment!("KRZ archive with extraneous preview image"),
                        ],
                    );
                }
                _ => {}
            }
        }

        // Sub-pass #7
        {
            for layer in kra_archive.all_layers_by_type(KraLayerType::PaintLayer) {
                if let Err(KraError::ColorProfileNotFound { .. }) = layer.color_profile(kra_archive) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            meta_comment!("Missing layer color profile"),
                            meta_layer!(layer),
                        ],
                    );
                }
            }
        }

        // Sub-pass #8
        {
            if let Err(KraError::ColorProfileNotFound { .. }) = kra_archive.main_doc.image.color_profile(kra_archive) {
                #[rustfmt::skip]
                lint_messages.push(
                    "Malformed document",
                    &[
                        meta_comment!("Missing document color profile"),
                    ],
                );
            }
        }

        Ok(())
    }
}
