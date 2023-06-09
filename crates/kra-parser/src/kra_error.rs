use std::io;

use camino_ext::FormattedPathBuf;
use derive_more::{Display, Error, From};
use strong_xml::XmlError;
use zip::result::ZipError;

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum KraError {
    #[display(fmt = "Cannot open KRA document \"{path:}\"")]
    ArchiveCannotOpen {
        path: FormattedPathBuf,
        source: io::Error,
    },

    #[display(fmt = "Cannot read KRA document \"{path:}\"")]
    ArchiveCannotRead {
        path: FormattedPathBuf,
        source: ZipError,
    },

    #[display(fmt = "Cannot find '{xml_path:}' in '{path:}'")]
    XmlNotFound {
        path: FormattedPathBuf,
        xml_path: String,
        source: ZipError,
    },

    #[display(fmt = "Cannot read '{xml_path:}' in '{path:}'")]
    XmlCannotRead {
        path: FormattedPathBuf,
        xml_path: String,
        source: io::Error,
    },

    #[display(fmt = "Cannot parse '{xml_path:}' in '{path:}'")]
    XmlCannotParse {
        path: FormattedPathBuf,
        xml_path: String,
        source: XmlError,
    },

    #[display(fmt = "Vector layer contents not found at '{svg_path:}'")]
    ContentSvgNotFound {
        svg_path: String,
    },

    #[display(fmt = "Color profile not found at '{color_profile_path:}'")]
    ColorProfileNotFound {
        color_profile_path: String,
    },

    #[display(fmt = "Mask file name field not found")]
    MaskFileNameFieldNotFound,

    #[display(fmt = "Filter config not found")]
    FilterConfigNotFound,

    #[display(fmt = "Cannot find filter config parameter '{param_name:}'")]
    CannotFindFilterConfigParam {
        param_name: String,
    },

    #[display(fmt = "Failed to parse filter config parameter '{param_name:}'")]
    FailedToParseFilterConfigParam {
        param_name: String,
    },

    #[from]
    ZipError(zip::result::ZipError),

    #[from]
    IoError(io::Error),

    #[from]
    XmlError(strong_xml::XmlError),
}
