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
        let zip_file = File::open(path).map_err(|io_error| KraError::ArchiveCannotOpen(io_error, path.to_owned()))?;

        let mut zip_archive = zip::ZipArchive::new(zip_file)
            .map_err(|zip_error| KraError::ArchiveCannotRead(zip_error, path.to_owned()))?;

        macro_rules! kra_xml {
            ($t:ident, $p:expr) => {{
                let file = zip_archive
                    .by_name($p)
                    .map_err(|zip_error| KraError::XmlNotFound(zip_error, path.to_owned(), $p))?;

                let data = io::read_to_string(file)
                    .map_err(|io_error| KraError::XmlCannotRead(io_error, path.to_owned(), $p))?;

                $t::from_str(&data).map_err(|xml_error| KraError::XmlCannotParse(xml_error, path.to_owned(), $p))?
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
