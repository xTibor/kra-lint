use std::io;

use derive_more::{Display, Error, From};

use crate::lint_config::LintConfigError;
use crate::lint_output::LintOutputError;

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintError {
    #[from]
    LintConfigError(LintConfigError),

    #[from]
    LintOutputError(LintOutputError),

    #[from]
    ZipError(zip::result::ZipError),

    #[from]
    IoError(io::Error),

    #[from]
    KraError(kra_parser::kra_error::KraError),
}
