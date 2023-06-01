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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[macro_export]
macro_rules! meta_layer {
    ($layer:expr) => {
        $crate::lint_metadata::LintMetadata::Layer {
            layer_name: $layer.name.to_string(),
            layer_uuid: $layer.uuid.to_string(),
        }
    };
}

#[macro_export]
macro_rules! meta_mask {
    ($mask:expr) => {
        $crate::lint_metadata::LintMetadata::Mask {
            mask_name: $mask.name.to_string(),
            mask_uuid: $mask.uuid.to_string(),
        }
    };
}

#[macro_export]
macro_rules! meta_expected {
    ($expected:expr) => {
        $crate::lint_metadata::LintMetadata::Expected($expected.to_string())
    };
}

#[macro_export]
macro_rules! meta_found {
    ($found:expr) => {
        $crate::lint_metadata::LintMetadata::Found($found.to_string())
    };
}

#[macro_export]
macro_rules! meta_missing_field {
    ($missing_field:expr) => {
        $crate::lint_metadata::LintMetadata::MissingField($missing_field.to_string())
    };
}

#[macro_export]
macro_rules! meta_bug {
    ($bug:expr) => {
        $crate::lint_metadata::LintMetadata::Bug($bug)
    };
}

#[macro_export]
macro_rules! meta_comment {
    ($comment:expr) => {
        $crate::lint_metadata::LintMetadata::Comment($comment.to_string())
    };
}

#[macro_export]
macro_rules! meta_error {
    ($error:expr) => {
        $crate::lint_metadata::LintMetadata::Error($error.to_string())
    };
}
