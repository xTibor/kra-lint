use std::fmt::Display;

use serde::Serialize;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LintMetadata::Layer { ref layer_name, .. } => {
                write!(f, "Layer: \"{}\"", layer_name.escape_debug())
            }
            LintMetadata::Mask { ref mask_name, .. } => {
                write!(f, "Mask: \"{}\"", mask_name.escape_debug())
            }
            LintMetadata::Expected(ref expected) => {
                write!(f, "Expected: \"{}\"", expected.trim_matches('"').escape_debug())
            }
            LintMetadata::Found(ref found) => {
                write!(f, "Found: \"{}\"", found.escape_debug())
            }
            LintMetadata::MissingField(ref missing_field) => {
                write!(f, "Missing field: {}", missing_field.escape_debug())
            }
            LintMetadata::Bug(ref bug) => {
                write!(f, "Bug {}", bug)
            }
            LintMetadata::Comment(ref comment) => {
                write!(f, "{}", comment)
            }
            LintMetadata::Error(ref error) => {
                write!(f, "Error: {}", error)
            }
        }
    }
}
