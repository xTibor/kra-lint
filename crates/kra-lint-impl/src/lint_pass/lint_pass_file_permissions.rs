#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config_fields::StringMatchExpression;
use crate::lint_output::lint_metadata_macros::{meta_expected, meta_found};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassFilePermissions {
    unix_permissions: Option<StringMatchExpression>,
}

impl LintPass for LintPassFilePermissions {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(unix_permissions) = self.unix_permissions.as_ref() {
                #[cfg(unix)]
                {
                    let kra_file_permissions = unix_mode::to_string(kra_archive.zip_path.metadata()?.mode());

                    if !unix_permissions.matches(&kra_file_permissions) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Incorrect UNIX file permissions",
                            &[
                                meta_expected!(unix_permissions),
                                meta_found!(kra_file_permissions),
                            ],
                        );
                    }
                }

                #[cfg(not(unix))]
                {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Platform doesn't support UNIX file permissions",
                        &[
                            meta_expected!("unix"),
                            meta_found!(std::env::consts::FAMILY),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
