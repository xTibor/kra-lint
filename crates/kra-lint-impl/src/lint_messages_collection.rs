use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use serde::Serialize;
use unicode_width::UnicodeWidthStr;

use crate::{LintMessages, LintOutputFormat};

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
    pub fn print(&self) {
        for (path, lint_messages) in &self.0 {
            for (lint_title, group) in &lint_messages.iter().group_by(|(lint_title, _)| lint_title) {
                let indent_size = path.to_string().width();
                let indent_str = format!("{}  | ", " ".repeat(indent_size));

                eprintln!("{}: {}", path, lint_title);
                for (_, lint_message) in group {
                    eprintln!("{}{}", indent_str, lint_message);
                }
                eprintln!();
            }
        }
    }

    pub fn format_output(&self, output_format: LintOutputFormat) -> String {
        match output_format {
            LintOutputFormat::PlainText => todo!(),
            LintOutputFormat::Toml => toml::ser::to_string_pretty(self).unwrap(),
            LintOutputFormat::Json => serde_json::to_string_pretty(self).unwrap(),
            LintOutputFormat::Ron => ron::to_string(self).unwrap(),
            LintOutputFormat::Yaml => serde_yaml::to_string(self).unwrap(),
        }
    }
}
