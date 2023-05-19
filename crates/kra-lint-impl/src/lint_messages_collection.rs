use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use serde::Serialize;
use unicode_width::UnicodeWidthStr;

use crate::{LintError, LintMessages, LintOutputFormat};

#[derive(Default, Serialize)]
pub struct LintMessagesCollection(Vec<(Utf8PathBuf, LintMessages)>);

impl LintMessagesCollection {
    pub(crate) fn push(&mut self, path: &Utf8Path, lint_messages: LintMessages) {
        self.0.push((path.to_owned(), lint_messages));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Utf8PathBuf, LintMessages)> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for LintMessagesCollection {
    type Item = (Utf8PathBuf, LintMessages);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl LintMessagesCollection {
    fn serialize_plain_text(&self) -> String {
        let mut result = String::new();

        for (path, lint_messages) in &self.0 {
            for (lint_title, group) in &lint_messages.iter().group_by(|(lint_title, _)| lint_title) {
                let indent_size = path.to_string().width();
                let indent_str = format!("{}  | ", " ".repeat(indent_size));

                result.push_str(&format!("{}: {}\n", path, lint_title));
                for (_, lint_message) in group {
                    result.push_str(&format!("{}{}\n", indent_str, lint_message));
                }
                result.push('\n');
            }
        }

        result
    }

    #[rustfmt::skip]
    pub fn format_output(&self, output_format: LintOutputFormat) -> Result<String, LintError> {
        match output_format {
            LintOutputFormat::PlainText => {
                Ok(self.serialize_plain_text())
            },
            LintOutputFormat::Toml => {
                toml::ser::to_string(self)
                    .map_err(LintError::FailedToSerializeTomlOutput)
            },
            LintOutputFormat::Json => {
                serde_json::to_string(self)
                    .map_err(LintError::FailedToSerializeJsonOutput)
            }
            LintOutputFormat::Ron => {
                ron::to_string(self)
                    .map_err(LintError::FailedToSerializeRonOutput)
            },
            LintOutputFormat::Yaml => {
                serde_yaml::to_string(self)
                    .map_err(LintError::FailedToSerializeYamlOutput)
            },
        }
    }
}
