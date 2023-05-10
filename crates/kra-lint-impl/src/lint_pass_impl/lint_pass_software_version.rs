use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{LintPass, LintPassResult, LintStringMatchExpression};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassSoftwareVersion {
    pub software_versions: LintStringMatchExpression,
}

impl LintPass for LintPassSoftwareVersion {
    fn lint(
        &self,
        kra_archive: &KraArchive,
        lint_messages: &mut Vec<String>,
    ) -> LintPassResult {
        // Sub-pass #1
        {
            let kra_software_version = &kra_archive.main_doc.software_version;

            if !self.software_versions.matches(kra_software_version) {
                lint_messages.push(format!(
                    "Incorrect software version (expected: {}, found: \"{}\")",
                    self.software_versions, kra_software_version
                ));
            }
        }

        Ok(())
    }
}
