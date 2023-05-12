use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassSoftwareVersion {
    software_versions: LintStringMatchExpression,
}

impl LintPass for LintPassSoftwareVersion {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
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
