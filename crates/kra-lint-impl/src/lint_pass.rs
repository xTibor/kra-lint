use kra_parser::kra_archive::KraArchive;

use crate::lint_error::LintError;
use crate::lint_messages::LintMessages;

pub(crate) type LintPassResult = Result<(), LintError>;

pub(crate) trait LintPass {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult;
}
