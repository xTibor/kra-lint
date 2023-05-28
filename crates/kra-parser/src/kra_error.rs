use std::io;

use camino::Utf8PathBuf;
use derive_more::{Display, Error, From};
use strong_xml::XmlError;
use zip::result::ZipError;

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum KraError {
    #[display(fmt = "Cannot open KRA document \"{path:}\"")]
    ArchiveCannotOpen {
        path: Utf8PathBuf,
        source: io::Error,
    },

    #[display(fmt = "Cannot read KRA document \"{path:}\"")]
    ArchiveCannotRead {
        path: Utf8PathBuf,
        source: ZipError,
    },

    #[display(fmt = "Cannot find '{xml_path:}' in '{path:}'")]
    XmlNotFound {
        path: Utf8PathBuf,
        xml_path: &'static str,
        source: ZipError,
    },

    #[display(fmt = "Cannot read '{xml_path:}' in '{path:}'")]
    XmlCannotRead {
        path: Utf8PathBuf,
        xml_path: &'static str,
        source: io::Error,
    },

    #[display(fmt = "Cannot parse '{xml_path:}' in '{path:}'")]
    XmlCannotParse {
        path: Utf8PathBuf,
        xml_path: &'static str,
        source: XmlError,
    },

    #[display(fmt = "Vector layer contents not found at '{svg_path:}'")]
    ContentSvgNotFound {
        svg_path: String,
    },

    #[display(fmt = "Layer color profile not found at '{color_profile_path:}'")]
    ColorProfileNotFound {
        color_profile_path: String,
    },

    #[from]
    ZipError(zip::result::ZipError),

    #[from]
    IoError(io::Error),
}
