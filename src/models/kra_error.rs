use std::path::PathBuf;
use std::{error, fmt, io};
use strong_xml::XmlError;
use zip::result::ZipError;

#[derive(Debug)]
pub enum KraError {
    ArchiveCannotOpen(io::Error, PathBuf),
    ArchiveCannotRead(ZipError, PathBuf),
    XmlNotFound(ZipError, PathBuf, &'static str),
    XmlCannotRead(io::Error, PathBuf, &'static str),
    XmlCannotParse(XmlError, PathBuf, &'static str),
}

impl error::Error for KraError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            KraError::ArchiveCannotOpen(ref err, _) => Some(err),
            KraError::ArchiveCannotRead(ref err, _) => Some(err),
            KraError::XmlNotFound(ref err, _, _) => Some(err),
            KraError::XmlCannotRead(ref err, _, _) => Some(err),
            KraError::XmlCannotParse(ref err, _, _) => Some(err),
        }
    }
}

impl fmt::Display for KraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KraError::ArchiveCannotOpen(_, ref path) => {
                write!(f, "Cannot open KRA archive '{}'", path.display())
            }
            KraError::ArchiveCannotRead(_, ref path) => {
                write!(f, "Cannot read KRA archive '{}'", path.display())
            }
            KraError::XmlNotFound(_, ref path, ref xml) => {
                write!(f, "Cannot find '{}' in '{}'", xml, path.display())
            }
            KraError::XmlCannotRead(_, ref path, ref xml) => {
                write!(f, "Cannot read '{}' in '{}'", xml, path.display())
            }
            KraError::XmlCannotParse(_, ref path, ref xml) => {
                write!(f, "Cannot parse '{}' in '{}'", xml, path.display())
            }
        }
    }
}
