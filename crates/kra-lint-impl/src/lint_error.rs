use std::{error, fmt, io};

use camino::Utf8PathBuf;

#[non_exhaustive]
#[derive(Debug)]
pub enum LintError {
    UnknownConfigFormat(String),
    ConfigNotFound(Utf8PathBuf),
    ConfigIncludeNotFound(Utf8PathBuf, Utf8PathBuf),
    FailedToReadConfig(Utf8PathBuf, io::Error),
    FailedToParseTomlConfig(Utf8PathBuf, toml::de::Error),
    FailedToParseJsonConfig(Utf8PathBuf, serde_json::Error),
    FailedToParseHjsonConfig(Utf8PathBuf, deser_hjson::Error),
    FailedToParseRonConfig(Utf8PathBuf, ron::error::SpannedError),
    FailedToParseYamlConfig(Utf8PathBuf, serde_yaml::Error),
    FailedToSerializeTomlConfig(toml::ser::Error),
    FailedToSerializeJsonConfig(serde_json::Error),
    FailedToSerializeRonConfig(ron::Error),
    FailedToSerializeYamlConfig(serde_yaml::Error),
    ZipError(zip::result::ZipError),
    IoError(io::Error),
    KraError(kra_parser::kra_error::KraError),
}

impl error::Error for LintError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            LintError::UnknownConfigFormat(_) => None,
            LintError::ConfigNotFound(_) => None,
            LintError::ConfigIncludeNotFound(_, _) => None,
            LintError::FailedToReadConfig(_, ref err) => Some(err),
            LintError::FailedToParseTomlConfig(_, ref err) => Some(err),
            LintError::FailedToParseJsonConfig(_, ref err) => Some(err),
            LintError::FailedToParseHjsonConfig(_, ref err) => Some(err),
            LintError::FailedToParseRonConfig(_, ref err) => Some(err),
            LintError::FailedToParseYamlConfig(_, ref err) => Some(err),
            LintError::FailedToSerializeTomlConfig(ref err) => Some(err),
            LintError::FailedToSerializeJsonConfig(ref err) => Some(err),
            LintError::FailedToSerializeRonConfig(ref err) => Some(err),
            LintError::FailedToSerializeYamlConfig(ref err) => Some(err),
            LintError::ZipError(ref err) => Some(err),
            LintError::IoError(ref err) => Some(err),
            LintError::KraError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for LintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LintError::UnknownConfigFormat(ref extension) => {
                write!(f, "Unknown config format \"{}\"", extension)
            }
            LintError::ConfigNotFound(ref path) => {
                write!(f, "Config file not found \"{}\"", path)
            }
            LintError::ConfigIncludeNotFound(ref path, ref included_from) => {
                write!(f, "Config include not found \"{}\" (included from: \"{}\")", path, included_from)
            }
            LintError::FailedToReadConfig(ref path, _) => {
                write!(f, "Failed to read config file \"{}\"", path)
            }
            LintError::FailedToParseTomlConfig(ref path, _) => {
                write!(f, "Failed to parse TOML config file \"{}\"", path)
            }
            LintError::FailedToParseJsonConfig(ref path, _) => {
                write!(f, "Failed to parse JSON config file \"{}\"", path)
            }
            LintError::FailedToParseHjsonConfig(ref path, _) => {
                write!(f, "Failed to parse Hjson config file \"{}\"", path)
            }
            LintError::FailedToParseRonConfig(ref path, _) => {
                write!(f, "Failed to parse RON config file \"{}\"", path)
            }
            LintError::FailedToParseYamlConfig(ref path, _) => {
                write!(f, "Failed to parse YAML config file \"{}\"", path)
            }
            LintError::FailedToSerializeTomlConfig(_) => {
                write!(f, "Failed to serialize TOML config")
            }
            LintError::FailedToSerializeJsonConfig(_) => {
                write!(f, "Failed to serialize JSON config")
            }
            LintError::FailedToSerializeRonConfig(_) => {
                write!(f, "Failed to serialize RON config")
            }
            LintError::FailedToSerializeYamlConfig(_) => {
                write!(f, "Failed to serialize YAML config")
            }
            LintError::ZipError(_) => {
                write!(f, "ZIP error")
            }
            LintError::IoError(_) => {
                write!(f, "I/O error")
            }
            LintError::KraError(_) => {
                write!(f, "KRA error")
            }
        }
    }
}

impl From<zip::result::ZipError> for LintError {
    fn from(zip_error: zip::result::ZipError) -> LintError {
        LintError::ZipError(zip_error)
    }
}

impl From<io::Error> for LintError {
    fn from(io_error: io::Error) -> LintError {
        LintError::IoError(io_error)
    }
}

impl From<kra_parser::kra_error::KraError> for LintError {
    fn from(kra_error: kra_parser::kra_error::KraError) -> LintError {
        LintError::KraError(kra_error)
    }
}
