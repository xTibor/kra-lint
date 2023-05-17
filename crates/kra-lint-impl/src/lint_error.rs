use std::io;

use camino::Utf8PathBuf;
use derive_more::{Display, Error, From};

#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintError {
    #[display(fmt = "Unknown config format \"{}\"", _0)]
    UnknownConfigFormat(#[error(not(source))] String),

    #[display(fmt = "Config file not found \"{}\"", _0)]
    ConfigNotFound(#[error(not(source))] Utf8PathBuf),

    #[display(fmt = "Config include not found \"{}\" (included from: \"{}\")", _0, _1)]
    ConfigIncludeNotFound(Utf8PathBuf, Utf8PathBuf),

    #[display(fmt = "Failed to read config file \"{}\"", _0)]
    FailedToReadConfig(Utf8PathBuf, #[error(source)] io::Error),

    #[display(fmt = "Failed to parse TOML config file \"{}\"", _0)]
    FailedToParseTomlConfig(Utf8PathBuf, #[error(source)] toml::de::Error),

    #[display(fmt = "Failed to parse JSON config file \"{}\"", _0)]
    FailedToParseJsonConfig(Utf8PathBuf, #[error(source)] serde_json::Error),

    #[display(fmt = "Failed to parse Hjson config file \"{}\"", _0)]
    FailedToParseHjsonConfig(Utf8PathBuf, #[error(source)] deser_hjson::Error),

    #[display(fmt = "Failed to parse RON config file \"{}\"", _0)]
    FailedToParseRonConfig(Utf8PathBuf, #[error(source)] ron::error::SpannedError),

    #[display(fmt = "Failed to parse YAML config file \"{}\"", _0)]
    FailedToParseYamlConfig(Utf8PathBuf, #[error(source)] serde_yaml::Error),

    #[display(fmt = "Failed to serialize TOML config")]
    FailedToSerializeTomlConfig(#[error(source)] toml::ser::Error),

    #[display(fmt = "Failed to serialize JSON config")]
    FailedToSerializeJsonConfig(#[error(source)] serde_json::Error),

    #[display(fmt = "Failed to serialize RON config")]
    FailedToSerializeRonConfig(#[error(source)] ron::Error),

    #[display(fmt = "Failed to serialize YAML config")]
    FailedToSerializeYamlConfig(#[error(source)] serde_yaml::Error),

    ZipError(#[error(source)] zip::result::ZipError),

    IoError(#[error(source)] io::Error),

    KraError(#[error(source)] kra_parser::kra_error::KraError),
}
