use std::io::Write;

use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use serde::Serialize;
use unicode_width::UnicodeWidthStr;

use crate::lint_output::{LintMessages, LintMessagesEntry, LintOutputError, LintOutputFormat};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Default, Serialize)]
pub struct LintMessagesCollectionEntry {
    pub path: Utf8PathBuf,
    pub messages: LintMessages,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[must_use = "lint results shouldn't be ignored"]
#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct LintMessagesCollection(Vec<LintMessagesCollectionEntry>);

impl LintMessagesCollection {
    pub(crate) fn push(&mut self, path: &Utf8Path, messages: LintMessages) {
        self.0.push(LintMessagesCollectionEntry { path: path.to_owned(), messages });
    }

    pub fn iter(&self) -> impl Iterator<Item = &LintMessagesCollectionEntry> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for LintMessagesCollection {
    type Item = LintMessagesCollectionEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl LintMessagesCollection {
    fn to_writer_plain_text<W>(&self, writer: &mut W) -> Result<(), LintOutputError>
    where
        W: Write,
    {
        for LintMessagesCollectionEntry { path, messages } in self.iter() {
            for (message_title, group) in
                &messages.iter().group_by(|LintMessagesEntry { message_title, .. }| message_title)
            {
                let indent_size = path.to_string().width();
                let indent_str = format!("{}  | ", " ".repeat(indent_size));

                writer.write_all(format!("{}: {}\n", path, message_title).as_bytes())?;
                for LintMessagesEntry { message_metadata, .. } in group {
                    writer.write_all(format!("{}{}\n", indent_str, message_metadata.iter().join(", ")).as_bytes())?;
                }
                writer.write_all(b"\n")?;
            }
        }

        Ok(())
    }

    #[rustfmt::skip]
    pub fn write_output<W>(&self, writer: &mut W, output_format: LintOutputFormat) -> Result<(), LintOutputError> where W: Write {
        match output_format {
            LintOutputFormat::PlainText => {
                self.to_writer_plain_text(writer)
            },
            LintOutputFormat::Json => {
                serde_json::to_writer(writer, self)
                    .map_err(LintOutputError::FailedToSerializeJsonOutput)
            }
            LintOutputFormat::Ron => {
                ron::ser::to_writer(writer, self)
                    .map_err(LintOutputError::FailedToSerializeRonOutput)
            },
            LintOutputFormat::Yaml => {
                serde_yaml::to_writer(writer, self)
                    .map_err(LintOutputError::FailedToSerializeYamlOutput)
            },
            LintOutputFormat::Pickle => {
                let pickle_options = serde_pickle::SerOptions::default();

                serde_pickle::to_writer(writer, self, pickle_options)
                    .map_err(LintOutputError::FailedToSerializePickleOutput)
            },
        }
    }
}
