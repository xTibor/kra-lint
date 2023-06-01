macro_rules! meta_layer {
    ($layer:expr) => {
        $crate::lint_output::LintMetadata::Layer {
            layer_name: $layer.name.to_string(),
            layer_uuid: $layer.uuid.to_string(),
        }
    };
}

macro_rules! meta_mask {
    ($mask:expr) => {
        $crate::lint_output::LintMetadata::Mask { mask_name: $mask.name.to_string(), mask_uuid: $mask.uuid.to_string() }
    };
}

macro_rules! meta_expected {
    ($expected:expr) => {
        $crate::lint_output::LintMetadata::Expected($expected.to_string())
    };
}

macro_rules! meta_found {
    ($found:expr) => {
        $crate::lint_output::LintMetadata::Found($found.to_string())
    };
}

macro_rules! meta_missing_field {
    ($missing_field:expr) => {
        $crate::lint_output::LintMetadata::MissingField($missing_field.to_string())
    };
}

macro_rules! meta_bug {
    ($bug:expr) => {
        $crate::lint_output::LintMetadata::Bug($bug)
    };
}

macro_rules! meta_comment {
    ($comment:expr) => {
        $crate::lint_output::LintMetadata::Comment($comment.to_string())
    };
}

macro_rules! meta_error {
    ($error:expr) => {
        $crate::lint_output::LintMetadata::Error($error.to_string())
    };
}

pub(crate) use {
    meta_bug, meta_comment, meta_error, meta_expected, meta_found, meta_layer, meta_mask, meta_missing_field,
};
