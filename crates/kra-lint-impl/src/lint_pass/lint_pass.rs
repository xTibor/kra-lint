use derive_more::{Display, Error, From};
use kra_parser::kra_archive::KraArchive;

use crate::lint_output::LintMessages;

#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintPassError {
    #[from]
    ZipError(zip::result::ZipError),

    #[from]
    IoError(std::io::Error),

    #[from]
    KraError(kra_parser::kra_error::KraError),
}

pub(crate) type LintPassResult = Result<(), LintPassError>;

pub(crate) trait LintPass {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult;
}
