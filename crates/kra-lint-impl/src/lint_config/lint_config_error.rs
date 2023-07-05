use std::io;

use camino_ext::FormattedPathBuf;
use derive_more::{Display, Error, From};

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintConfigError {
    #[display(fmt = "Unknown config format \"{extension:}\" of config file \"{path:}\"")]
    UnknownConfigFormat {
        path: FormattedPathBuf,
        extension: String,
    },

    #[display(fmt = "Config file not found \"{path:}\"")]
    ConfigNotFound {
        #[error(not(source))]
        path: FormattedPathBuf,
    },

    #[display(fmt = "Config include not found \"{path:}\" (included from: \"{included_from:}\")")]
    ConfigIncludeNotFound {
        path: FormattedPathBuf,
        included_from: FormattedPathBuf,
    },

    #[display(fmt = "Failed to open config file \"{path:}\"")]
    FailedToOpenConfig {
        path: FormattedPathBuf,
        source: io::Error,
    },

    #[display(fmt = "Failed to create config file \"{path:}\"")]
    FailedToCreateConfig {
        path: FormattedPathBuf,
        source: io::Error,
    },

    #[display(fmt = "Failed to parse TOML config file \"{path:}\"")]
    FailedToParseTomlConfig {
        path: FormattedPathBuf,
        source: toml::de::Error,
    },

    #[display(fmt = "Failed to parse JSON config file \"{path:}\"")]
    FailedToParseJsonConfig {
        path: FormattedPathBuf,
        source: serde_json::Error,
    },

    #[display(fmt = "Failed to parse Hjson config file \"{path:}\"")]
    FailedToParseHjsonConfig {
        path: FormattedPathBuf,
        source: deser_hjson::Error,
    },

    #[display(fmt = "Failed to parse RON config file \"{path:}\"")]
    FailedToParseRonConfig {
        path: FormattedPathBuf,
        source: ron::error::SpannedError,
    },

    #[display(fmt = "Failed to parse YAML config file \"{path:}\"")]
    FailedToParseYamlConfig {
        path: FormattedPathBuf,
        source: serde_yaml::Error,
    },

    #[display(fmt = "Failed to parse Pickle config file \"{path:}\"")]
    FailedToParsePickleConfig {
        path: FormattedPathBuf,
        source: serde_pickle::Error,
    },

    #[display(fmt = "Failed to parse Gura config file \"{path:}\"")]
    FailedToParseGuraConfig {
        path: FormattedPathBuf,
        source: serde_gura::Error,
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

    #[display(fmt = "Failed to serialize Gura config")]
    FailedToSerializeGuraConfig(serde_gura::Error),

    #[from]
    IoError(io::Error),
}
