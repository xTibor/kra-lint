use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::{LintMessages, LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassSoftwareVersion {
    software_name: Option<LintStringMatchExpression>,
    software_version: Option<LintStringMatchExpression>,
    syntax_version: Option<LintStringMatchExpression>,
}

impl LintPass for LintPassSoftwareVersion {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(software_name) = self.software_name.as_ref() {
                let kra_software_name = &kra_archive.main_doc.editor;

                if !software_name.matches(kra_software_name) {
                    lint_messages.push(
                        "Incorrect software name",
                        format!("Expected: {}, Found: \"{}\"", software_name, kra_software_name.escape_debug()),
                    );
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(software_version) = self.software_version.as_ref() {
                let kra_software_version = &kra_archive.main_doc.software_version;

                if !software_version.matches(kra_software_version) {
                    lint_messages.push(
                        "Incorrect software version",
                        format!("Expected: {}, Found: \"{}\"", software_version, kra_software_version.escape_debug()),
                    );
                }
            }
        }

        // Sub-pass #3
        {
            if let Some(syntax_version) = self.syntax_version.as_ref() {
                let kra_syntax_version = &kra_archive.main_doc.syntax_version;

                if !syntax_version.matches(kra_syntax_version) {
                    lint_messages.push(
                        "Incorrect document syntax version",
                        format!("Expected: {}, Found: \"{}\"", syntax_version, kra_syntax_version.escape_debug()),
                    );
                }
            }
        }

        Ok(())
    }
}
