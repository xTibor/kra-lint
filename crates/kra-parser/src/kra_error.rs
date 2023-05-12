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
        }
    }
}
