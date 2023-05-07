use serde::Deserialize;

use crate::models::kra_archive::KraArchive;
use crate::models::kra_maindoc::{KraMainDocLayer, KraMainDocMask};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type LintPassResult = Vec<String>;

pub trait LintPass {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult;
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

mod lint_animation;
mod lint_colorspace;
mod lint_copyright;
mod lint_document_name;
mod lint_document_size;
mod lint_file_layers;
mod lint_hidden_surface;
mod lint_layer_styles;
mod lint_malformed_document;
mod lint_non_default_blending;
mod lint_prohibit_compositions;
mod lint_prohibit_custom_palettes;
mod lint_prohibit_kseexpr;
mod lint_software_version;
mod lint_surface_names;
mod lint_surface_type;

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintConfig {
    pub lint_animation:                Option<lint_animation               ::LintPassAnimation             >,
    pub lint_colorspace:               Option<lint_colorspace              ::LintPassColorspace            >,
    pub lint_copyright:                Option<lint_copyright               ::LintPassCopyright             >,
    pub lint_document_name:            Option<lint_document_name           ::LintPassDocumentName          >,
    pub lint_document_size:            Option<lint_document_size           ::LintPassDocumentSize          >,
    pub lint_file_layers:              Option<lint_file_layers             ::LintPassFileLayers            >,
    pub lint_hidden_surface:           Option<lint_hidden_surface          ::LintPassHiddenSurface         >,
    pub lint_layer_styles:             Option<lint_layer_styles            ::LintPassLayerStyles           >,
    pub lint_malformed_document:       Option<lint_malformed_document      ::LintPassMalformedDocument     >,
    pub lint_non_default_blending:     Option<lint_non_default_blending    ::LintPassNonDefaultBlending    >,
    pub lint_prohibit_compositions:    Option<lint_prohibit_compositions   ::LintPassProhibitCompositions  >,
    pub lint_prohibit_custom_palettes: Option<lint_prohibit_custom_palettes::LintPassProhibitCustomPalettes>,
    pub lint_prohibit_kseexpr:         Option<lint_prohibit_kseexpr        ::LintPassProhibitKSeExpr       >,
    pub lint_software_version:         Option<lint_software_version        ::LintPassSoftwareVersion       >,
    pub lint_surface_names:            Option<lint_surface_names           ::LintPassSurfaceNames          >,
    pub lint_surface_type:             Option<lint_surface_type            ::LintPassSurfaceType           >,
}

impl LintPass for LintConfig {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        macro_rules! lint_pass {
            ($lint_name:ident) => {{
                if let Some($lint_name) = self.$lint_name.as_ref() {
                    results.extend($lint_name.lint(kra_archive))
                }
            }};
        }

        lint_pass!(lint_animation);
        lint_pass!(lint_colorspace);
        lint_pass!(lint_copyright);
        lint_pass!(lint_document_name);
        lint_pass!(lint_document_size);
        lint_pass!(lint_file_layers);
        lint_pass!(lint_hidden_surface);
        lint_pass!(lint_layer_styles);
        lint_pass!(lint_malformed_document);
        lint_pass!(lint_non_default_blending);
        lint_pass!(lint_prohibit_compositions);
        lint_pass!(lint_prohibit_custom_palettes);
        lint_pass!(lint_prohibit_kseexpr);
        lint_pass!(lint_software_version);
        lint_pass!(lint_surface_names);
        lint_pass!(lint_surface_type);

        results
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum LintLayerTypeFlags<T> {
    All(Option<T>),
    ByType {
        paint_layers:  Option<T>,
        group_layers:  Option<T>,
        clone_layers:  Option<T>,
        vector_layers: Option<T>,
        filter_layers: Option<T>,
        fill_layers:   Option<T>,
        file_layers:   Option<T>,
    },
}

impl<T> LintLayerTypeFlags<T> {
    #[rustfmt::skip]
    pub fn get<'a>(&'a self, layer: &KraMainDocLayer) -> (&'a Option<T>, &str) {
        match *self {
            LintLayerTypeFlags::All(ref all_layers) => (all_layers, "layer"),
            LintLayerTypeFlags::ByType {
                ref paint_layers,
                ref group_layers,
                ref clone_layers,
                ref vector_layers,
                ref filter_layers,
                ref fill_layers,
                ref file_layers,
            } => match layer.node_type.as_str() {
                "paintlayer"      => (paint_layers,  "paint layer" ),
                "grouplayer"      => (group_layers,  "group layer" ),
                "clonelayer"      => (clone_layers,  "clone layer" ),
                "shapelayer"      => (vector_layers, "vector layer"),
                "adjustmentlayer" => (filter_layers, "filter layer"),
                "generatorlayer"  => (fill_layers,   "fill layer"  ),
                "filelayer"       => (file_layers,   "file layer"  ),
                _ => unreachable!("Unknown layer node type: \"{}\"", layer.node_type),
            },
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum LintMaskTypeFlags<T> {
    All(Option<T>),
    ByType {
        transparency_masks: Option<T>,
        filter_masks:       Option<T>,
        colorize_masks:     Option<T>,
        transform_masks:    Option<T>,
        local_selections:   Option<T>,
    },
}

impl<T> LintMaskTypeFlags<T> {
    #[rustfmt::skip]
    pub fn get<'a>(&'a self, mask: &KraMainDocMask) -> (&'a Option<T>, &str) {
        match *self {
            LintMaskTypeFlags::All(ref all_masks) => (all_masks, "mask"),
            LintMaskTypeFlags::ByType {
                ref transparency_masks,
                ref filter_masks,
                ref colorize_masks,
                ref transform_masks,
                ref local_selections,
            } => match mask.node_type.as_str() {
                "transparencymask" => (transparency_masks, "transparency mask"),
                "filtermask"       => (filter_masks,       "filter mask"      ),
                "colorizemask"     => (colorize_masks,     "colorize mask"    ),
                "transformmask"    => (transform_masks,    "transform mask"   ),
                "selectionmask"    => (local_selections,   "local selection"  ),
                _ => unreachable!("Unknown mask node type: \"{}\"", mask.node_type),
            },
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum StringMatchExpression {
    FullMatch(String),
    Regex {
        #[serde(rename = "regex")]
        pattern: String,
    },
    StartsWith {
        #[serde(rename = "starts_with")]
        pattern: String,
    },
    EndsWith {
        #[serde(rename = "ends_with")]
        pattern: String,
    },
    Contains {
        #[serde(rename = "contains")]
        pattern: String,
    },
    BinaryOr {
        #[serde(rename = "or")]
        expressions: Vec<StringMatchExpression>,
    },
    BinaryAnd {
        #[serde(rename = "and")]
        expressions: Vec<StringMatchExpression>,
    },
    BinaryNot {
        #[serde(rename = "not")]
        expression: Box<StringMatchExpression>,
    },
}

impl StringMatchExpression {
    pub fn matches(&self, value: &str) -> bool {
        match self {
            StringMatchExpression::FullMatch(pattern) => value == pattern,
            StringMatchExpression::Regex { pattern } => {
                let compiled_regex = regex::Regex::new(pattern)
                    .expect("Failed to compile regular expression");
                compiled_regex.is_match(value)
            }
            StringMatchExpression::StartsWith { pattern } => {
                value.starts_with(pattern)
            }
            StringMatchExpression::EndsWith { pattern } => {
                value.ends_with(pattern)
            }
            StringMatchExpression::Contains { pattern } => {
                value.contains(pattern)
            }
            StringMatchExpression::BinaryOr { expressions } => {
                expressions.iter().any(|expression| expression.matches(value))
            }
            StringMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(value))
            }
            StringMatchExpression::BinaryNot { expression } => {
                !expression.matches(value)
            }
        }
    }
}

impl std::fmt::Display for StringMatchExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringMatchExpression::FullMatch(pattern) => {
                write!(f, "\"{}\"", pattern)
            }
            StringMatchExpression::Regex { pattern } => {
                write!(f, "regex(\"{}\")", pattern)
            }
            StringMatchExpression::StartsWith { pattern } => {
                write!(f, "starts_with(\"{}\")", pattern)
            }
            StringMatchExpression::EndsWith { pattern } => {
                write!(f, "ends_with(\"{}\")", pattern)
            }
            StringMatchExpression::Contains { pattern } => {
                write!(f, "contains(\"{}\")", pattern)
            }
            StringMatchExpression::BinaryOr { expressions } => {
                let param_list = expressions
                    .iter()
                    .map(StringMatchExpression::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "or({})", param_list)
            }
            StringMatchExpression::BinaryAnd { expressions } => {
                let param_list = expressions
                    .iter()
                    .map(StringMatchExpression::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "and({})", param_list)
            }
            StringMatchExpression::BinaryNot { expression } => {
                write!(f, "not({})", expression)
            }
        }
    }
}
