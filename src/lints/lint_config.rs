use camino::Utf8PathBuf;
use serde::Deserialize;

use crate::lints::{lint_pass_impl, LintPass, LintPassResult};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintIncludes {
    pub paths: Vec<Utf8PathBuf>,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintConfig {
    pub includes:                 Option<LintIncludes>,

    pub animation:                Option<lint_pass_impl::animation               ::LintPassAnimation             >,
    pub colorspace:               Option<lint_pass_impl::colorspace              ::LintPassColorspace            >,
    pub copyright:                Option<lint_pass_impl::copyright               ::LintPassCopyright             >,
    pub document_name:            Option<lint_pass_impl::document_name           ::LintPassDocumentName          >,
    pub document_size:            Option<lint_pass_impl::document_size           ::LintPassDocumentSize          >,
    pub file_layers:              Option<lint_pass_impl::file_layers             ::LintPassFileLayers            >,
    pub hidden_surface:           Option<lint_pass_impl::hidden_surface          ::LintPassHiddenSurface         >,
    pub layer_styles:             Option<lint_pass_impl::layer_styles            ::LintPassLayerStyles           >,
    pub malformed_document:       Option<lint_pass_impl::malformed_document      ::LintPassMalformedDocument     >,
    pub non_default_blending:     Option<lint_pass_impl::non_default_blending    ::LintPassNonDefaultBlending    >,
    pub prohibit_compositions:    Option<lint_pass_impl::prohibit_compositions   ::LintPassProhibitCompositions  >,
    pub prohibit_custom_palettes: Option<lint_pass_impl::prohibit_custom_palettes::LintPassProhibitCustomPalettes>,
    pub prohibit_kseexpr:         Option<lint_pass_impl::prohibit_kseexpr        ::LintPassProhibitKSeExpr       >,
    pub software_version:         Option<lint_pass_impl::software_version        ::LintPassSoftwareVersion       >,
    pub surface_names:            Option<lint_pass_impl::surface_names           ::LintPassSurfaceNames          >,
    pub surface_type:             Option<lint_pass_impl::surface_type            ::LintPassSurfaceType           >,
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

        lint_pass!(animation);
        lint_pass!(colorspace);
        lint_pass!(copyright);
        lint_pass!(document_name);
        lint_pass!(document_size);
        lint_pass!(file_layers);
        lint_pass!(hidden_surface);
        lint_pass!(layer_styles);
        lint_pass!(malformed_document);
        lint_pass!(non_default_blending);
        lint_pass!(prohibit_compositions);
        lint_pass!(prohibit_custom_palettes);
        lint_pass!(prohibit_kseexpr);
        lint_pass!(software_version);
        lint_pass!(surface_names);
        lint_pass!(surface_type);

        results
    }
}
