use std::io::{Read, Seek};

use zip::read::ZipFile;
use zip::result::{ZipError, ZipResult};
use zip::ZipArchive;

pub trait ZipArchiveExt {
    fn exists(&mut self, path: &str) -> ZipResult<bool>;

    // https://github.com/zip-rs/zip/discussions/341
    fn by_name_opt<'a>(&'a mut self, path: &str) -> ZipResult<Option<ZipFile<'a>>>;

    fn read_to_string(&mut self, path: &str) -> ZipResult<Option<String>>;
}

impl<R> ZipArchiveExt for ZipArchive<R>
where
    R: Read + Seek,
{
    fn exists(&mut self, path: &str) -> ZipResult<bool> {
        match self.by_name(path) {
            Ok(_) => Ok(true),
            Err(ZipError::FileNotFound) => Ok(false),
            Err(err) => Err(err),
        }
    }

    fn by_name_opt<'a>(&'a mut self, path: &str) -> ZipResult<Option<ZipFile<'a>>> {
        match self.by_name(path) {
            Ok(zip_file) => Ok(Some(zip_file)),
            Err(ZipError::FileNotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn read_to_string(&mut self, path: &str) -> ZipResult<Option<String>> {
        if let Some(zip_file) = self.by_name_opt(path)? {
            Ok(Some(std::io::read_to_string(zip_file)?))
        } else {
            Ok(None)
        }
    }
}
