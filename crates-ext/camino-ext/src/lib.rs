use std::fmt;

use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug)]
pub struct FormattedPathBuf(Utf8PathBuf);

impl fmt::Display for FormattedPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.strip_cwd_prefix().fmt(f)
    }
}

impl FormattedPathBuf {
    fn strip_cwd_prefix(&self) -> Utf8PathBuf {
        let cwd_prefix = std::env::current_dir().expect("Failed to get current working directory");

        if let Ok(path) = self.0.strip_prefix(cwd_prefix) {
            path.into()
        } else {
            self.0.to_path_buf()
        }
    }
}

impl<P: AsRef<Utf8Path>> From<P> for FormattedPathBuf {
    fn from(path: P) -> Self {
        FormattedPathBuf(path.as_ref().to_path_buf())
    }
}
