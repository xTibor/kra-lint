use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult, LintStringMatchExpression};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassSoftwareVersion {
    pub software_versions: LintStringMatchExpression,
}

impl LintPass for LintPassSoftwareVersion {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            let kra_software_version = &kra_archive.main_doc.software_version;

            if !self.software_versions.matches(kra_software_version) {
                results.push(format!(
                    "Incorrect software version (expected: {}, found: \"{}\")",
                    self.software_versions, kra_software_version
                ));
            }
        }

        results
    }
}
