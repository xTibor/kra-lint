use std::fmt::Display;

use serde::Serialize;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[must_use = "lint results shouldn't be ignored"]
#[derive(Default, Serialize)]
pub struct LintMessages {
    messages: Vec<(String, Vec<LintMetadata>)>,
}

impl LintMessages {
    pub(crate) fn push<S>(&mut self, lint_title: S, lint_message_metadata: &[LintMetadata])
    where
        S: AsRef<str> + Into<String>,
    {
        self.messages.push((lint_title.into(), lint_message_metadata.to_vec()));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, Vec<LintMetadata>)> {
        self.messages.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub(crate) fn sort_and_dedup(&mut self) {
        self.messages.sort();
        self.messages.dedup();
    }
}

impl IntoIterator for LintMessages {
    type Item = (String, Vec<LintMetadata>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[non_exhaustive]
#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "meta_type", content = "meta_data")]
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
