use std::io;

use derive_more::{Display, Error, From};

#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintPassError {
    #[from]
    ZipError(zip::result::ZipError),

    #[from]
    IoError(io::Error),

    #[from]
    KraError(kra_parser::kra_error::KraError),
}
