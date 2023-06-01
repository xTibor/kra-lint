use std::io;

use derive_more::{Display, Error, From};

use crate::lint_config::lint_config_error::LintConfigError;

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintError {
    #[display(fmt = "Failed to serialize TOML output")]
    FailedToSerializeTomlOutput(toml::ser::Error),

    #[display(fmt = "Failed to serialize JSON output")]
    FailedToSerializeJsonOutput(serde_json::Error),

    #[display(fmt = "Failed to serialize RON output")]
    FailedToSerializeRonOutput(ron::Error),

    #[display(fmt = "Failed to serialize YAML output")]
    FailedToSerializeYamlOutput(serde_yaml::Error),

    #[display(fmt = "Failed to serialize Pickle output")]
    FailedToSerializePickleOutput(serde_pickle::Error),

    #[from]
    LintConfigError(LintConfigError),

    #[from]
    ZipError(zip::result::ZipError),

    #[from]
    IoError(io::Error),

    #[from]
    KraError(kra_parser::kra_error::KraError),
}
