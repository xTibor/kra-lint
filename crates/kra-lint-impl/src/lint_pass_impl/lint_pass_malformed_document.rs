use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use kra_parser::kra_maindoc::KraLayerType;

use crate::lint_pass::{LintPass, LintPassResult};
use crate::LintMessages;

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
                lint_messages.push("Malformed document", "Unescaped documentinfo.xml <abstract> tag, Bug 446376");
            }
        }

        // Sub-pass #2
        {
            let zip_archive = kra_archive.zip_archive.borrow();

            if zip_archive.file_names().any(|file_name| file_name.contains("../")) {
                lint_messages.push("Malformed document", "Path traversal vulnerability, Bug 429925");
            }
        }

        // Sub-pass #3
        {
            for layer in kra_archive.all_layers_by_type(KraLayerType::CloneLayer) {
                if let Some(clone_from_uuid) = layer.clone_from_uuid.as_ref() {
                    if !kra_archive.all_layers().any(|target_layer| &target_layer.uuid == clone_from_uuid) {
                        lint_messages.push(
                            "Malformed document",
                            format!(
                                "Missing clone layer target layer, Layer: \"{}\", Bug 414699",
                                layer.name.escape_debug()
                            ),
                        );
                    }
                } else {
                    lint_messages.push(
                        "Malformed document",
                        format!("Missing clone layer target field, Layer: \"{}\"", layer.name.escape_debug()),
                    );
                }
            }
        }

        // Sub-pass #4
        {
            if let Some(composition_container) = kra_archive.main_doc.image.composition_container.as_ref() {
                if composition_container.into_iter().any(|composition| composition.name.contains('/')) {
                    lint_messages.push("Malformed document", "Compositions path traversal vulnerability");
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
                        lint_messages.push(
                            "Malformed document",
                            format!("Clone layer loop, Layer: \"{}\"", layer.name.escape_debug()),
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

        Ok(())
    }
}
