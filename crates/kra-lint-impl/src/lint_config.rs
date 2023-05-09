use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

use kra_parser::kra_archive::KraArchive;

use crate::{lint_pass_impl, LintPass, LintPassResult};

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

    pub animation:                Option<lint_pass_impl::lint_pass_animation               ::LintPassAnimation             >,
    pub colorspace:               Option<lint_pass_impl::lint_pass_colorspace              ::LintPassColorspace            >,
    pub copyright:                Option<lint_pass_impl::lint_pass_copyright               ::LintPassCopyright             >,
    pub document_name:            Option<lint_pass_impl::lint_pass_document_name           ::LintPassDocumentName          >,
    pub document_size:            Option<lint_pass_impl::lint_pass_document_size           ::LintPassDocumentSize          >,
    pub file_layers:              Option<lint_pass_impl::lint_pass_file_layers             ::LintPassFileLayers            >,
    pub hidden_surface:           Option<lint_pass_impl::lint_pass_hidden_surface          ::LintPassHiddenSurface         >,
    pub layer_styles:             Option<lint_pass_impl::lint_pass_layer_styles            ::LintPassLayerStyles           >,
    pub malformed_document:       Option<lint_pass_impl::lint_pass_malformed_document      ::LintPassMalformedDocument     >,
    pub non_default_blending:     Option<lint_pass_impl::lint_pass_non_default_blending    ::LintPassNonDefaultBlending    >,
    pub prohibit_compositions:    Option<lint_pass_impl::lint_pass_prohibit_compositions   ::LintPassProhibitCompositions  >,
    pub prohibit_custom_palettes: Option<lint_pass_impl::lint_pass_prohibit_custom_palettes::LintPassProhibitCustomPalettes>,
    pub prohibit_kseexpr:         Option<lint_pass_impl::lint_pass_prohibit_kseexpr        ::LintPassProhibitKSeExpr       >,
    pub software_version:         Option<lint_pass_impl::lint_pass_software_version        ::LintPassSoftwareVersion       >,
    pub surface_names:            Option<lint_pass_impl::lint_pass_surface_names           ::LintPassSurfaceNames          >,
    pub surface_type:             Option<lint_pass_impl::lint_pass_surface_type            ::LintPassSurfaceType           >,
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

impl LintConfig {
    pub fn from_path(lint_config_path: &Utf8Path) -> LintConfig {
        let lint_config_str = std::fs::read_to_string(lint_config_path)
            .expect("Failed to read config file");

        match lint_config_path.extension().map(str::to_lowercase).as_deref() {
            None | Some("toml") => toml::from_str(&lint_config_str)
                .expect("Failed to parse config file"),
            Some("json" | "hjson") => deser_hjson::from_str(&lint_config_str)
                .expect("Failed to parse config file"),
            Some("ron") => {
                let ron_options = ron::Options::default()
                    .with_default_extension(
                        ron::extensions::Extensions::IMPLICIT_SOME,
                    );

                ron_options
                    .from_str(&lint_config_str)
                    .expect("Failed to parse config file")
            }
            Some("yaml") => serde_yaml::from_str(&lint_config_str)
                .expect("Failed to parse config file"),
            Some(ext) => panic!("Unknown config file format \"{}\"", ext),
        }
    }
}
