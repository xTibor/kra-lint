use std::io::Write;

use camino::{Utf8Path, Utf8PathBuf};
use derive_more::IntoIterator;
use serde::Serialize;

use crate::lint_output::{LintMessages, LintOutputError, LintOutputFormat};

#[derive(Default, Serialize)]
pub struct LintMessagesCollectionEntry {
    pub path: Utf8PathBuf,
    pub messages: LintMessages,
}

#[rustfmt::skip]
#[must_use = "lint results shouldn't be ignored"]
#[derive(Default, Serialize, IntoIterator)]
#[serde(transparent)]
pub struct LintMessagesCollection (
    #[into_iterator(ref)]
    Vec<LintMessagesCollectionEntry>
);

impl LintMessagesCollection {
    pub(crate) fn push(&mut self, path: &Utf8Path, messages: LintMessages) {
        self.0.push(LintMessagesCollectionEntry { path: path.to_owned(), messages });
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn message_count(&self) -> usize {
        self.0.iter().map(|entry| entry.messages.message_count()).sum()
    }
}

impl LintMessagesCollection {
    #[allow(unused_variables)]
    #[rustfmt::skip]
    pub fn write_output<W>(&self, writer: &mut W, output_format: LintOutputFormat) -> Result<(), LintOutputError> where W: Write {
        match output_format {
            #[cfg(feature = "output-plaintext")]
            LintOutputFormat::PlainText => {
                crate::lint_output::lint_output_plaintext::to_writer(writer, self)
            }

            #[cfg(feature = "output-json")]
            LintOutputFormat::Json => {
                serde_json::to_writer(writer, self)
                    .map_err(LintOutputError::FailedToSerializeJsonOutput)
            }

            #[cfg(feature = "output-ron")]
            LintOutputFormat::Ron => {
                ron::ser::to_writer(writer, self)
                    .map_err(LintOutputError::FailedToSerializeRonOutput)
            }

            #[cfg(feature = "output-yaml")]
            LintOutputFormat::Yaml => {
                serde_yaml::to_writer(writer, self)
                    .map_err(LintOutputError::FailedToSerializeYamlOutput)
            }

            #[cfg(feature = "output-pickle")]
            LintOutputFormat::Pickle => {
                let pickle_options = serde_pickle::SerOptions::default();

                serde_pickle::to_writer(writer, self, pickle_options)
                    .map_err(LintOutputError::FailedToSerializePickleOutput)
            }

            #[cfg(feature = "output-gura")]
            LintOutputFormat::Gura => {
                // TODO: serde_gura::to_writer (https://github.com/gura-conf/serde-gura)
                let tmp_string = serde_gura::to_string(self)
                    .map_err(LintOutputError::FailedToSerializeGuraOutput)?;
                Ok(writer.write_all(tmp_string.as_bytes())?)
            }
        }
    }
}
