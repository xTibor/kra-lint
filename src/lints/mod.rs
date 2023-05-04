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
mod lint_document_resolution;
mod lint_document_size;
mod lint_hidden_surface;
mod lint_layer_styles;
mod lint_malformed_document;
mod lint_non_default_blending;
mod lint_prohibit_custom_palettes;
mod lint_prohibit_kseexpr;
mod lint_prohibit_surface_names;
mod lint_software_version;
mod lint_surface_type;

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintConfig {
    pub lint_animation:                Option<lint_animation               ::LintPassAnimation             >,
    pub lint_colorspace:               Option<lint_colorspace              ::LintPassColorspace            >,
    pub lint_copyright:                Option<lint_copyright               ::LintPassCopyright             >,
    pub lint_document_resolution:      Option<lint_document_resolution     ::LintPassDocumentResolution    >,
    pub lint_document_size:            Option<lint_document_size           ::LintPassDocumentSize          >,
    pub lint_hidden_surface:           Option<lint_hidden_surface          ::LintPassHiddenSurface         >,
    pub lint_malformed_document:       Option<lint_malformed_document      ::LintPassMalformedDocument     >,
    pub lint_non_default_blending:     Option<lint_non_default_blending    ::LintPassNonDefaultBlending    >,
    pub lint_prohibit_custom_palettes: Option<lint_prohibit_custom_palettes::LintPassProhibitCustomPalettes>,
    pub lint_prohibit_kseexpr:         Option<lint_prohibit_kseexpr        ::LintPassProhibitKSeExpr       >,
    pub lint_prohibit_surface_names:   Option<lint_prohibit_surface_names  ::LintPassProhibitSurfaceNames  >,
    pub lint_software_version:         Option<lint_software_version        ::LintPassSoftwareVersion       >,
    pub lint_surface_type:             Option<lint_surface_type            ::LintPassSurfaceType           >,
    pub lint_layer_styles:             Option<lint_layer_styles            ::LintPassLayerStyles           >,
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
        lint_pass!(lint_document_resolution);
        lint_pass!(lint_document_size);
        lint_pass!(lint_hidden_surface);
        lint_pass!(lint_malformed_document);
        lint_pass!(lint_non_default_blending);
        lint_pass!(lint_prohibit_custom_palettes);
        lint_pass!(lint_prohibit_kseexpr);
        lint_pass!(lint_prohibit_surface_names);
        lint_pass!(lint_software_version);
        lint_pass!(lint_surface_type);
        lint_pass!(lint_layer_styles);

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
