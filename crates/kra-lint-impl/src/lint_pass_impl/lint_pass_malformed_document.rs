use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassMalformedDocument {}

impl LintPass for LintPassMalformedDocument {
    fn lint(
        &self,
        kra_archive: &KraArchive,
        results: &mut Vec<String>,
    ) -> LintPassResult {
        // Sub-pass #1
        {
            let mut zip_archive = kra_archive.zip_archive.borrow_mut();
            let zip_file = zip_archive.by_name("documentinfo.xml").unwrap();

            let documentinfo_xml = std::io::read_to_string(zip_file).unwrap();

            if documentinfo_xml.matches("]]>").count() > 1 {
                results.push(
                    "Potentially malformed document (Unescaped documentinfo.xml <abstract> tag, Bug 446376)".to_owned(),
                );
            }
        }

        // Sub-pass #2
        {
            let zip_archive = kra_archive.zip_archive.borrow();

            if zip_archive
                .file_names()
                .any(|file_name| file_name.contains("../"))
            {
                results.push("Malformed document (Path traversal vulnerability, Bug 429925)".to_owned());
            }
        }

        // Sub-pass #3
        {
            for layer in kra_archive.all_layers() {
                if layer.node_type == "clonelayer" {
                    if let Some(clone_from_uuid) =
                        layer.clone_from_uuid.as_ref()
                    {
                        if !kra_archive.all_layers().any(|target_layer| {
                            &target_layer.uuid == clone_from_uuid
                        }) {
                            results.push(format!(
                                "Malformed document (Missing clone layer target layer, layer: \"{}\", Bug 414699)",
                                layer.name
                            ));
                        }
                    } else {
                        results.push(format!(
                            "Malformed document (Missing clone layer target field, layer: \"{}\")",
                            layer.name
                        ));
                    }
                }
            }
        }

        // Sub-pass #4
        {
            if let Some(composition_container) =
                kra_archive.main_doc.image.composition_container.as_ref()
            {
                if composition_container
                    .compositions
                    .iter()
                    .any(|composition| composition.name.contains('/'))
                {
                    results.push("Malformed document (Compositions path traversal vulnerability)".to_owned());
                }
            }
        }

        // Sub-pass #5
        {
            for layer in kra_archive.all_layers() {
                if layer.node_type == "clonelayer" {
                    let uuid_root = &layer.uuid;

                    let mut uuid_todo: Vec<&String> = vec![uuid_root];
                    let mut uuid_done: Vec<&String> = vec![];

                    while !uuid_todo.is_empty() {
                        let current_uuid = uuid_todo.pop().unwrap();
                        uuid_done.push(current_uuid);

                        #[rustfmt::skip]
                        let referencing_uuid = kra_archive
                            .all_layers()
                            .filter(|from_layer| from_layer.clone_from_uuid.as_ref() == Some(current_uuid))
                            .map(|from_layer| &from_layer.uuid)
                            .collect::<Vec<_>>();

                        if referencing_uuid.contains(&uuid_root) {
                            results.push(format!(
                                "Malformed document (Clone layer loop, layer: \"{}\")",
                                layer.name
                            ));
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
        }

        Ok(())
    }
}
