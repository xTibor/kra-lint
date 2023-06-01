use std::io;

use camino::Utf8PathBuf;
use derive_more::{Display, Error, From};

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintConfigError {
    #[display(fmt = "Unknown config format \"{extension:}\"")]
    UnknownConfigFormat {
        #[error(not(source))]
        extension: String,
    },

    #[display(fmt = "Config file not found \"{path:}\"")]
    ConfigNotFound {
        #[error(not(source))]
        path: Utf8PathBuf,
    },

    #[display(fmt = "Config include not found \"{path:}\" (included from: \"{included_from:}\")")]
    ConfigIncludeNotFound {
        path: Utf8PathBuf,
        included_from: Utf8PathBuf,
    },

    #[display(fmt = "Failed to open config file \"{path:}\"")]
    FailedToOpenConfig {
        path: Utf8PathBuf,
        source: io::Error,
    },

    #[display(fmt = "Failed to create config file \"{path:}\"")]
    FailedToCreateConfig {
        path: Utf8PathBuf,
        source: io::Error,
    },

    #[display(fmt = "Failed to parse TOML config file \"{path:}\"")]
    FailedToParseTomlConfig {
        path: Utf8PathBuf,
        source: toml::de::Error,
    },

    #[display(fmt = "Failed to parse JSON config file \"{path:}\"")]
    FailedToParseJsonConfig {
        path: Utf8PathBuf,
        source: serde_json::Error,
    },

    #[display(fmt = "Failed to parse Hjson config file \"{path:}\"")]
    FailedToParseHjsonConfig {
        path: Utf8PathBuf,
        source: deser_hjson::Error,
    },

    #[display(fmt = "Failed to parse RON config file \"{path:}\"")]
    FailedToParseRonConfig {
        path: Utf8PathBuf,
        source: ron::error::SpannedError,
    },

    #[display(fmt = "Failed to parse YAML config file \"{path:}\"")]
    FailedToParseYamlConfig {
        path: Utf8PathBuf,
        source: serde_yaml::Error,
    },

    #[display(fmt = "Failed to parse Pickle config file \"{path:}\"")]
    FailedToParsePickleConfig {
        path: Utf8PathBuf,
        source: serde_pickle::Error,
    },

    #[display(fmt = "Failed to serialize TOML config")]
    FailedToSerializeTomlConfig(toml::ser::Error),

    #[display(fmt = "Failed to serialize JSON config")]
    FailedToSerializeJsonConfig(serde_json::Error),

    #[display(fmt = "Failed to serialize RON config")]
    FailedToSerializeRonConfig(ron::Error),

    #[display(fmt = "Failed to serialize YAML config")]
    FailedToSerializeYamlConfig(serde_yaml::Error),

    #[display(fmt = "Failed to serialize Pickle config")]
    FailedToSerializePickleConfig(serde_pickle::Error),

    #[from]
    IoError(io::Error),
}
