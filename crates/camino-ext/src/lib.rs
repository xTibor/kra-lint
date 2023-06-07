use camino::{Utf8Path, Utf8PathBuf};

pub trait Utf8PathExt {
    fn strip_cwd_prefix(&self) -> Utf8PathBuf;
}

impl Utf8PathExt for Utf8Path {
    fn strip_cwd_prefix(&self) -> Utf8PathBuf {
        let cwd_prefix = std::env::current_dir().expect("Failed to get current working directory");

        if let Ok(path) = self.strip_prefix(cwd_prefix) {
            path.into()
        } else {
            self.to_path_buf()
        }
    }
}
