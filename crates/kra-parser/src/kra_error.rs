use std::io;

use camino::Utf8PathBuf;
use derive_more::{Display, Error, From};
use strong_xml::XmlError;
use zip::result::ZipError;

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum KraError {
    #[display(fmt = "Cannot open KRA document \"{}\"", _1)]
    ArchiveCannotOpen (
        #[error(source)] io::Error,
        Utf8PathBuf,
    ),

    #[display(fmt = "Cannot read KRA document \"{}\"", _1)]
    ArchiveCannotRead (
        #[error(source)] ZipError,
        Utf8PathBuf,
    ),

    #[display(fmt = "Cannot find '{}' in '{}'", _2, _1)]
    XmlNotFound(
        #[error(source)] ZipError,
        Utf8PathBuf,
        &'static str,
    ),

    #[display(fmt = "Cannot read '{}' in '{}'", _2, _1)]
    XmlCannotRead (
        #[error(source)] io::Error,
        Utf8PathBuf,
        &'static str,
    ),

    #[display(fmt = "Cannot parse '{}' in '{}'", _2, _1)]
    XmlCannotParse (
        #[error(source)] XmlError,
        Utf8PathBuf,
        &'static str,
    ),

    ZipError (
        #[error(source)] zip::result::ZipError,
    ),

    IoError (
        #[error(source)] io::Error,
    ),
}
