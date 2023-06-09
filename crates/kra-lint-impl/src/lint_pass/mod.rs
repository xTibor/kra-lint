mod lint_pass;

pub use lint_pass::LintPassError;
pub(crate) use lint_pass::{LintPass, LintPassResult};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub(crate) mod lint_pass_animation;
pub(crate) mod lint_pass_audio_track;
pub(crate) mod lint_pass_colorize_mask;
pub(crate) mod lint_pass_colorspace;
pub(crate) mod lint_pass_copyright;
pub(crate) mod lint_pass_document_name;
pub(crate) mod lint_pass_document_size;
pub(crate) mod lint_pass_document_structure;
pub(crate) mod lint_pass_file_layers;
pub(crate) mod lint_pass_file_name;
pub(crate) mod lint_pass_file_permissions;
pub(crate) mod lint_pass_filters;
pub(crate) mod lint_pass_hidden_surface;
pub(crate) mod lint_pass_layer_styles;
pub(crate) mod lint_pass_malformed_document;
pub(crate) mod lint_pass_non_default_blending;
pub(crate) mod lint_pass_prohibit_compositions;
pub(crate) mod lint_pass_prohibit_custom_palettes;
pub(crate) mod lint_pass_prohibit_kseexpr;
pub(crate) mod lint_pass_software_version;
pub(crate) mod lint_pass_surface_names;
pub(crate) mod lint_pass_surface_type;
pub(crate) mod lint_pass_vector_layers;
