use std::{error, fmt, io};

use camino::Utf8PathBuf;
use strong_xml::XmlError;
use zip::result::ZipError;

#[non_exhaustive]
#[derive(Debug)]
pub enum KraError {
    ArchiveCannotOpen(io::Error, Utf8PathBuf),
    ArchiveCannotRead(ZipError, Utf8PathBuf),
    XmlNotFound(ZipError, Utf8PathBuf, &'static str),
    XmlCannotRead(io::Error, Utf8PathBuf, &'static str),
    XmlCannotParse(XmlError, Utf8PathBuf, &'static str),
    UnknownLayerNodeType(String),
    UnknownMaskNodeType(String),
    UnknownColorLabel(String),
    ZipError(zip::result::ZipError),
    IoError(io::Error),
}

impl error::Error for KraError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            KraError::ArchiveCannotOpen(ref err, _) => Some(err),
            KraError::ArchiveCannotRead(ref err, _) => Some(err),
            KraError::XmlNotFound(ref err, _, _) => Some(err),
            KraError::XmlCannotRead(ref err, _, _) => Some(err),
            KraError::XmlCannotParse(ref err, _, _) => Some(err),
            KraError::UnknownLayerNodeType(_) => None,
            KraError::UnknownMaskNodeType(_) => None,
            KraError::UnknownColorLabel(_) => None,
            KraError::ZipError(ref err) => Some(err),
            KraError::IoError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for KraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KraError::ArchiveCannotOpen(_, ref path) => {
                write!(f, "Cannot open KRA document \"{}\"", path)
            }
            KraError::ArchiveCannotRead(_, ref path) => {
                write!(f, "Cannot read KRA document \"{}\"", path)
            }
            KraError::XmlNotFound(_, ref path, ref xml) => {
                write!(f, "Cannot find '{}' in '{}'", xml, path)
            }
            KraError::XmlCannotRead(_, ref path, ref xml) => {
                write!(f, "Cannot read '{}' in '{}'", xml, path)
            }
            KraError::XmlCannotParse(_, ref path, ref xml) => {
                write!(f, "Cannot parse '{}' in '{}'", xml, path)
            }
            KraError::UnknownLayerNodeType(ref node_type) => {
                write!(f, "Unknown layer node type \"{}\"", node_type)
            }
            KraError::UnknownMaskNodeType(ref node_type) => {
                write!(f, "Unknown mask node type \"{}\"", node_type)
            }
            KraError::UnknownColorLabel(ref color_label) => {
                write!(f, "Unknown color label \"{}\"", color_label)
            }
            KraError::ZipError(_) => {
                write!(f, "ZIP error")
            }
            KraError::IoError(_) => {
                write!(f, "I/O error")
            }
        }
    }
}

impl From<zip::result::ZipError> for KraError {
    fn from(zip_error: zip::result::ZipError) -> KraError {
        KraError::ZipError(zip_error)
    }
}

impl From<io::Error> for KraError {
    fn from(io_error: io::Error) -> KraError {
        KraError::IoError(io_error)
    }
}
