use std::io::Write;

use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use serde::Serialize;
use unicode_width::UnicodeWidthStr;

use crate::{LintError, LintMessages, LintOutputFormat};

#[derive(Default, Serialize)]
pub struct LintMessagesCollection {
    message_collection: Vec<(Utf8PathBuf, LintMessages)>,
}

impl LintMessagesCollection {
    pub(crate) fn push(&mut self, path: &Utf8Path, lint_messages: LintMessages) {
        self.message_collection.push((path.to_owned(), lint_messages));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Utf8PathBuf, LintMessages)> {
        self.message_collection.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.message_collection.is_empty()
    }
}

impl IntoIterator for LintMessagesCollection {
    type Item = (Utf8PathBuf, LintMessages);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.message_collection.into_iter()
    }
}

impl LintMessagesCollection {
    fn to_writer_plain_text<W>(&self, writer: &mut W) -> Result<(), LintError>
    where
        W: Write,
    {
        for (path, lint_messages) in &self.message_collection {
            for (lint_title, group) in &lint_messages.iter().group_by(|(lint_title, _)| lint_title) {
                let indent_size = path.to_string().width();
                let indent_str = format!("{}  | ", " ".repeat(indent_size));

                writer.write_all(format!("{}: {}\n", path, lint_title).as_bytes())?;
                for (_, lint_message) in group {
                    writer.write_all(format!("{}{}\n", indent_str, lint_message).as_bytes())?;
                }
                writer.write_all(b"\n")?;
            }
        }

        Ok(())
    }

    #[rustfmt::skip]
    pub fn write_output<W>(&self, writer: &mut W, output_format: LintOutputFormat) -> Result<(), LintError> where W: Write {
        match output_format {
            LintOutputFormat::PlainText => {
                self.to_writer_plain_text(writer)
            },
            LintOutputFormat::Toml => {
                // TODO: toml::to_writer (https://github.com/toml-rs/toml/pull/349)
                let toml_output = toml::ser::to_string(self)
                    .map_err(LintError::FailedToSerializeTomlOutput)?;
                Ok(writer.write_all(toml_output.as_bytes())?)
            },
            LintOutputFormat::Json => {
                serde_json::to_writer(writer, self)
                    .map_err(LintError::FailedToSerializeJsonOutput)
            }
            LintOutputFormat::Ron => {
                ron::ser::to_writer(writer, self)
                    .map_err(LintError::FailedToSerializeRonOutput)
            },
            LintOutputFormat::Yaml => {
                serde_yaml::to_writer(writer, self)
                    .map_err(LintError::FailedToSerializeYamlOutput)
            },
            LintOutputFormat::Pickle => {
                let pickle_options = serde_pickle::SerOptions::default();
                serde_pickle::to_writer(writer, self, pickle_options)
                    .map_err(LintError::FailedToSerializePickleOutput)
            },
        }
    }
}
