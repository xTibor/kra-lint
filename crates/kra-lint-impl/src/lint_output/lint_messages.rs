use derive_more::IntoIterator;
use serde::Serialize;

use crate::lint_output::LintMetadata;

#[derive(Default, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LintMessagesEntry {
    pub message_title: String,
    pub message_metadata: Vec<LintMetadata>,
}

#[rustfmt::skip]
#[must_use = "lint results shouldn't be ignored"]
#[derive(Default, Serialize, IntoIterator)]
#[serde(transparent)]
pub struct LintMessages (
    #[into_iterator(ref)]
    Vec<LintMessagesEntry>
);

impl LintMessages {
    pub(crate) fn push<S>(&mut self, message_title: S, message_metadata: &[LintMetadata])
    where
        S: AsRef<str> + Into<String>,
    {
        self.0.push(LintMessagesEntry {
            message_title: message_title.into(),
            message_metadata: message_metadata.to_vec(),
        });
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn sort_and_dedup(&mut self) {
        self.0.sort();
        self.0.dedup();
    }
}
