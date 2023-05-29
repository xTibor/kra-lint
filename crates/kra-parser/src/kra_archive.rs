use std::cell::RefCell;
use std::fs::File;
use std::io;

use camino::{Utf8Path, Utf8PathBuf};
use strong_xml::XmlRead;
use zip::ZipArchive;

use crate::kra_documentinfo::KraDocumentInfo;
use crate::kra_error::KraError;
use crate::kra_maindoc::KraMainDoc;

#[derive(Debug)]
pub struct KraArchive {
    pub document_info: KraDocumentInfo,
    pub main_doc: KraMainDoc,
    pub zip_archive: RefCell<ZipArchive<File>>,
    pub zip_path: Utf8PathBuf,
}

impl KraArchive {
    pub fn from_path(path: &Utf8Path) -> Result<Self, KraError> {
        let zip_file =
            File::open(path).map_err(|source| KraError::ArchiveCannotOpen { path: path.to_owned(), source })?;

        let mut zip_archive = zip::ZipArchive::new(zip_file)
            .map_err(|source| KraError::ArchiveCannotRead { path: path.to_owned(), source })?;

        macro_rules! kra_xml {
            ($xml_type:ident, $xml_path:expr) => {{
                let file = zip_archive.by_name($xml_path).map_err(|source| KraError::XmlNotFound {
                    path: path.to_owned(),
                    xml_path: $xml_path.to_owned(),
                    source,
                })?;

                let data = io::read_to_string(file).map_err(|source| KraError::XmlCannotRead {
                    path: path.to_owned(),
                    xml_path: $xml_path.to_owned(),
                    source,
                })?;

                $xml_type::from_str(&data).map_err(|source| KraError::XmlCannotParse {
                    path: path.to_owned(),
                    xml_path: $xml_path.to_owned(),
                    source,
                })?
            }};
        }

        Ok(KraArchive {
            document_info: kra_xml!(KraDocumentInfo, "documentinfo.xml"),
            main_doc: kra_xml!(KraMainDoc, "maindoc.xml"),
            zip_archive: RefCell::new(zip_archive),
            zip_path: path.to_owned(),
        })
    }
}
