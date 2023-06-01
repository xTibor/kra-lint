use kra_parser::kra_archive::KraArchive;

use crate::lint_output::LintMessages;
use crate::lint_pass::LintPassError;

pub(crate) type LintPassResult = Result<(), LintPassError>;

pub(crate) trait LintPass {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult;
}
