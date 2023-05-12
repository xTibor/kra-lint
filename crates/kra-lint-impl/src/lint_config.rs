use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::{lint_pass_impl, LintError, LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintIncludes {
    pub(crate) paths: Vec<Utf8PathBuf>,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LintConfig {
    pub(crate) includes:      Option<LintIncludes>,

    animation:                Option<lint_pass_impl::lint_pass_animation               ::LintPassAnimation             >,
    colorspace:               Option<lint_pass_impl::lint_pass_colorspace              ::LintPassColorspace            >,
    copyright:                Option<lint_pass_impl::lint_pass_copyright               ::LintPassCopyright             >,
    document_name:            Option<lint_pass_impl::lint_pass_document_name           ::LintPassDocumentName          >,
    document_size:            Option<lint_pass_impl::lint_pass_document_size           ::LintPassDocumentSize          >,
    file_layers:              Option<lint_pass_impl::lint_pass_file_layers             ::LintPassFileLayers            >,
    file_name:                Option<lint_pass_impl::lint_pass_file_name               ::LintPassFileName              >,
    hidden_surface:           Option<lint_pass_impl::lint_pass_hidden_surface          ::LintPassHiddenSurface         >,
    layer_styles:             Option<lint_pass_impl::lint_pass_layer_styles            ::LintPassLayerStyles           >,
    malformed_document:       Option<lint_pass_impl::lint_pass_malformed_document      ::LintPassMalformedDocument     >,
    non_default_blending:     Option<lint_pass_impl::lint_pass_non_default_blending    ::LintPassNonDefaultBlending    >,
    prohibit_compositions:    Option<lint_pass_impl::lint_pass_prohibit_compositions   ::LintPassProhibitCompositions  >,
    prohibit_custom_palettes: Option<lint_pass_impl::lint_pass_prohibit_custom_palettes::LintPassProhibitCustomPalettes>,
    prohibit_kseexpr:         Option<lint_pass_impl::lint_pass_prohibit_kseexpr        ::LintPassProhibitKSeExpr       >,
    software_version:         Option<lint_pass_impl::lint_pass_software_version        ::LintPassSoftwareVersion       >,
    surface_names:            Option<lint_pass_impl::lint_pass_surface_names           ::LintPassSurfaceNames          >,
    surface_type:             Option<lint_pass_impl::lint_pass_surface_type            ::LintPassSurfaceType           >,
    vector_layers:            Option<lint_pass_impl::lint_pass_vector_layers           ::LintPassVectorLayers          >,
}

impl LintPass for LintConfig {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut Vec<String>) -> LintPassResult {
        macro_rules! lint_pass {
            ($lint_name:ident) => {{
                if let Some($lint_name) = self.$lint_name.as_ref() {
                    $lint_name.lint(kra_archive, lint_messages)?;
                }
            }};
        }

        lint_pass!(animation);
        lint_pass!(colorspace);
        lint_pass!(copyright);
        lint_pass!(document_name);
        lint_pass!(document_size);
        lint_pass!(file_layers);
        lint_pass!(file_name);
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
        lint_pass!(vector_layers);

        Ok(())
    }
}

impl LintConfig {
    #[rustfmt::skip]
    pub fn load_from_path(lint_config_path: &Utf8Path) -> Result<LintConfig, LintError> {
        if !lint_config_path.is_file() {
            return Err(LintError::ConfigNotFound(lint_config_path.to_owned()));
        }

        let lint_config_str = std::fs::read_to_string(lint_config_path)
            .map_err(|io_error| LintError::FailedToReadConfig(lint_config_path.to_owned(), io_error))?;

        match lint_config_path.extension().map(str::to_lowercase).as_deref() {
            None | Some("toml") => {
                toml::from_str(&lint_config_str)
                    .map_err(|toml_error| LintError::FailedToParseTomlConfig(lint_config_path.to_owned(), toml_error))
            }
            Some("json") => {
                serde_json::from_str(&lint_config_str)
                    .map_err(|json_error| LintError::FailedToParseJsonConfig(lint_config_path.to_owned(), json_error))
            }
            Some("ron") => {
                let ron_options = ron::Options::default()
                    .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);

                ron_options
                    .from_str(&lint_config_str)
                    .map_err(|ron_error| LintError::FailedToParseRonConfig(lint_config_path.to_owned(), ron_error))
            }
            Some("yaml" | "yml") => {
                serde_yaml::from_str(&lint_config_str)
                    .map_err(|yaml_error| LintError::FailedToParseYamlConfig(lint_config_path.to_owned(), yaml_error))
            }
            Some(extension) => {
                Err(LintError::UnknownConfigFormat(extension.to_owned()))
            }
        }
    }

    #[rustfmt::skip]
    pub fn save_to_path(&self, lint_config_path: &Utf8Path) -> Result<(), LintError> {
        let lint_config_str = match lint_config_path.extension().map(str::to_lowercase).as_deref() {
            None | Some("toml") => {
                let lint_config_str = toml::to_string(&self)
                    .map_err(LintError::FailedToSerializeTomlConfig)?;
                Ok(lint_config_str)
            },
            Some("json") => {
                let lint_config_str = serde_json::to_string_pretty(&self)
                    .map_err(LintError::FailedToSerializeJsonConfig)?;
                Ok(lint_config_str)
            }
            Some("ron") => {
                let ron_options = ron::Options::default()
                    .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
                let ron_pretty_config = ron::ser::PrettyConfig::default();

                let lint_config_str = ron_options.to_string_pretty(&self, ron_pretty_config)
                    .map_err(LintError::FailedToSerializeRonConfig)?;
                Ok(lint_config_str)
            }
            Some("yaml" | "yml") => {
                let lint_config_str = serde_yaml::to_string(&self)
                    .map_err(LintError::FailedToSerializeYamlConfig)?;
                Ok(lint_config_str)
            }
            Some(extension) => {
                Err(LintError::UnknownConfigFormat(extension.to_owned()))
            }
        }?;

        std::fs::write(lint_config_path, lint_config_str)?;
        Ok(())
    }
}
