use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;
use std_ext::MetadataExt;

use crate::lint_config_fields::StringMatchExpression;
use crate::lint_output::lint_metadata_macros::{meta_expected, meta_found};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassFilePermissions {
    unix_permissions: StringMatchExpression,
}

impl LintPass for LintPassFilePermissions {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1 - TODO: Unix only
        {
            let kra_file_permissions = kra_archive.zip_path.metadata()?.mode_symbolic();

            if !self.unix_permissions.matches(&kra_file_permissions) {
                #[rustfmt::skip]
                lint_messages.push(
                    "Incorrect file permissions",
                    &[
                        meta_expected!(self.unix_permissions),
                        meta_found!(kra_file_permissions),
                    ],
                );
            }
        }

        Ok(())
    }
}
