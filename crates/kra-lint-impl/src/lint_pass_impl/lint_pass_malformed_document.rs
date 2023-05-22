use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::KraLayerType;

use crate::lint_pass::{LintPass, LintPassResult};
use crate::{LintMessages, LintMetadata};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassMalformedDocument {}

impl LintPass for LintPassMalformedDocument {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            let mut zip_archive = kra_archive.zip_archive.borrow_mut();
            let zip_file = zip_archive.by_name("documentinfo.xml")?;

            let documentinfo_xml = std::io::read_to_string(zip_file)?;

            if documentinfo_xml.matches("]]>").count() > 1 {
                #[rustfmt::skip]
                lint_messages.push(
                    "Malformed document",
                    &[
                        LintMetadata::Comment("Unescaped documentinfo.xml <abstract> tag".to_owned()),
                        LintMetadata::Bug(446376),
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
                        LintMetadata::Comment("Path traversal vulnerability".to_owned()),
                        LintMetadata::Bug(429925),
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
                                LintMetadata::Comment("Missing clone layer target layer".to_owned()),
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
                                LintMetadata::Bug(414699),
                            ],
                        );
                    }
                } else {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            LintMetadata::Comment("Missing clone layer target field".to_owned()),
                            LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
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
                            LintMetadata::Comment("Compositions path traversal vulnerability".to_owned()),
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
                                LintMetadata::Comment("Clone layer loop".to_owned()),
                                LintMetadata::Layer { layer_name: layer.name.to_string(), layer_uuid: layer.uuid.to_string() },
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

            let contains_mergedimage_png = zip_archive.by_name("mergedimage.png").is_ok();
            let file_extension = kra_archive.zip_path.extension().map(str::to_lowercase);

            match (file_extension.as_deref(), contains_mergedimage_png) {
                (Some("kra"), false) => {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            LintMetadata::Comment("KRA archive without required preview image".to_owned()),
                        ],
                    );
                }
                (Some("krz"), true) => {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Malformed document",
                        &[
                            LintMetadata::Comment("KRZ archive with extraneous preview image".to_owned()),
                        ],
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }
}
