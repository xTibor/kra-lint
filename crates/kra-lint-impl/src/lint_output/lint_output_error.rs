use std::io;

use derive_more::{Display, Error, From};

#[non_exhaustive]
#[derive(Debug, Display, Error, From)]
pub enum LintOutputError {
    #[cfg(feature = "output-json")]
    #[display(fmt = "Failed to serialize JSON output")]
    FailedToSerializeJsonOutput(serde_json::Error),

    #[cfg(feature = "output-ron")]
    #[display(fmt = "Failed to serialize RON output")]
    FailedToSerializeRonOutput(ron::Error),

    #[cfg(feature = "output-yaml")]
    #[display(fmt = "Failed to serialize YAML output")]
    FailedToSerializeYamlOutput(serde_yaml::Error),

    #[cfg(feature = "output-pickle")]
    #[display(fmt = "Failed to serialize Pickle output")]
    FailedToSerializePickleOutput(serde_pickle::Error),

    #[cfg(feature = "output-gura")]
    #[display(fmt = "Failed to serialize Gura output")]
    FailedToSerializeGuraOutput(serde_gura::Error),

    #[from]
    IoError(io::Error),
}
