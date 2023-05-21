use std::fmt::Display;

use serde::Serialize;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Default, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LintMessagesEntry {
    pub message_title: String,
    pub message_metadata: Vec<LintMetadata>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[must_use = "lint results shouldn't be ignored"]
#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct LintMessages(Vec<LintMessagesEntry>);

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

    pub fn iter(&self) -> impl Iterator<Item = &LintMessagesEntry> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn sort_and_dedup(&mut self) {
        self.0.sort();
        self.0.dedup();
    }
}

impl IntoIterator for LintMessages {
    type Item = LintMessagesEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[non_exhaustive]
#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "metadata_type", content = "metadata_content")]
#[serde(rename_all = "snake_case")]
pub enum LintMetadata {
    Layer { layer_name: String, layer_uuid: String },
    Mask { mask_name: String, mask_uuid: String },
    Expected(String),
    Found(String),
    MissingField(String),
    Bug(usize),
    Comment(String),
    Error(String),
}

impl Display for LintMetadata {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LintMetadata::Layer { ref layer_name, .. } => {
                write!(f, "Layer: \"{}\"", layer_name.escape_debug())
            },
            LintMetadata::Mask { ref mask_name, .. } => {
                write!(f, "Mask: \"{}\"", mask_name.escape_debug())
            },
            LintMetadata::Expected(ref expected) => {
                write!(f, "Expected: \"{}\"", expected.trim_matches('"').escape_debug())
            },
            LintMetadata::Found(ref found) => {
                write!(f, "Found: \"{}\"", found.escape_debug())
            },
            LintMetadata::MissingField(ref missing_field) => {
                write!(f, "Missing field: {}", missing_field.escape_debug())
            },
            LintMetadata::Bug(ref bug) => {
                write!(f, "Bug {}", bug)
            },
            LintMetadata::Comment(ref comment) => {
                write!(f, "{}", comment)
            },
            LintMetadata::Error(ref error) => {
                write!(f, "Error: {}", error)
            },
        }
    }
}
