use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_output::lint_messages::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_expected, meta_found};

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
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect software name",
                        &[
                            meta_expected!(software_name),
                            meta_found!(kra_software_name),
                        ],
                    );
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(software_version) = self.software_version.as_ref() {
                let kra_software_version = &kra_archive.main_doc.software_version;

                if !software_version.matches(kra_software_version) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect software version",
                        &[
                            meta_expected!(software_version),
                            meta_found!(kra_software_version),
                        ],
                    );
                }
            }
        }

        // Sub-pass #3
        {
            if let Some(syntax_version) = self.syntax_version.as_ref() {
                let kra_syntax_version = &kra_archive.main_doc.syntax_version;

                if !syntax_version.matches(kra_syntax_version) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect document syntax version",
                        &[
                            meta_expected!(syntax_version),
                            meta_found!(kra_syntax_version),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
