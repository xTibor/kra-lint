use std::io;

use derive_more::{Display, Error, From};

#[rustfmt::skip]
#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintOutputError {
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
    IoError(io::Error),
}
