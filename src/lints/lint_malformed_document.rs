use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassMalformedDocument {}

impl LintPass for LintPassMalformedDocument {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

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

        results
    }
}
