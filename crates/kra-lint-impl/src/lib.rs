#![allow(clippy::module_inception)]

mod lint_config;
mod lint_config_fields;
mod lint_output;
mod lint_pass;

pub use lint_config::{LintConfig, LintConfigCollection};
pub use lint_output::{
    LintMessages, LintMessagesCollection, LintMessagesCollectionEntry, LintMessagesEntry, LintMetadata,
    LintOutputFormat,
};

pub use lint_config::LintConfigError;
pub use lint_output::LintOutputError;
pub use lint_pass::LintPassError;
