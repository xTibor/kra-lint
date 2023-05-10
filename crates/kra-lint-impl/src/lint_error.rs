use std::{error, fmt, io};

use camino::Utf8PathBuf;

#[derive(Debug)]
pub enum LintError {
    UnknownConfigFormat(String),
    FailedToReadConfig(Utf8PathBuf, io::Error),
    FailedToParseTomlConfig(Utf8PathBuf, toml::de::Error),
    FailedToParseHjsonConfig(Utf8PathBuf, deser_hjson::Error),
    FailedToParseRonConfig(Utf8PathBuf, ron::error::SpannedError),
    FailedToParseYamlConfig(Utf8PathBuf, serde_yaml::Error),
    UnknownLayerNodeType(String),
    UnknownMaskNodeType(String),
}

impl error::Error for LintError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            LintError::UnknownConfigFormat(_) => None,
            LintError::FailedToReadConfig(_, ref err) => Some(err),
            LintError::FailedToParseTomlConfig(_, ref err) => Some(err),
            LintError::FailedToParseHjsonConfig(_, ref err) => Some(err),
            LintError::FailedToParseRonConfig(_, ref err) => Some(err),
            LintError::FailedToParseYamlConfig(_, ref err) => Some(err),
            LintError::UnknownLayerNodeType(_) => None,
            LintError::UnknownMaskNodeType(_) => None,
        }
    }
}

impl fmt::Display for LintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LintError::UnknownConfigFormat(ref extension) => {
                write!(f, "Unknown config format \"{}\"", extension)
            }
            LintError::FailedToReadConfig(ref path, _) => {
                write!(f, "Failed to read config file \"{}\"", path)
            }
            LintError::FailedToParseTomlConfig(ref path, _) => {
                write!(f, "Failed to parse TOML config file \"{}\"", path)
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
            LintError::UnknownLayerNodeType(ref node_type) => {
                write!(f, "Unknown layer node type \"{}\"", node_type)
            }
            LintError::UnknownMaskNodeType(ref node_type) => {
                write!(f, "Unknown mask node type \"{}\"", node_type)
            }
        }
    }
}
