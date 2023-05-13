use kra_parser::kra_archive::KraArchive;

use crate::lint_error::LintError;
use crate::LintMessages;

pub type LintPassResult = Result<(), LintError>;

pub trait LintPass {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult;
}
