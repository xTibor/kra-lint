use std::fs::File;
use std::io::Write;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config::LintConfigError;
use crate::lint_output::macros::meta_error;
use crate::lint_output::LintMessages;
use crate::lint_pass::{self, LintPass, LintPassResult};

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

    animation:                Option<lint_pass::lint_pass_animation               ::LintPassAnimation             >,
    colorize_mask:            Option<lint_pass::lint_pass_colorize_mask           ::LintPassColorizeMask          >,
    colorspace:               Option<lint_pass::lint_pass_colorspace              ::LintPassColorspace            >,
    copyright:                Option<lint_pass::lint_pass_copyright               ::LintPassCopyright             >,
    document_name:            Option<lint_pass::lint_pass_document_name           ::LintPassDocumentName          >,
    document_size:            Option<lint_pass::lint_pass_document_size           ::LintPassDocumentSize          >,
    document_structure:       Option<lint_pass::lint_pass_document_structure      ::LintPassDocumentStructure     >,
    file_layers:              Option<lint_pass::lint_pass_file_layers             ::LintPassFileLayers            >,
    file_name:                Option<lint_pass::lint_pass_file_name               ::LintPassFileName              >,
    filters:                  Option<lint_pass::lint_pass_filters                 ::LintPassFilters               >,
    hidden_surface:           Option<lint_pass::lint_pass_hidden_surface          ::LintPassHiddenSurface         >,
    layer_styles:             Option<lint_pass::lint_pass_layer_styles            ::LintPassLayerStyles           >,
    malformed_document:       Option<lint_pass::lint_pass_malformed_document      ::LintPassMalformedDocument     >,
    non_default_blending:     Option<lint_pass::lint_pass_non_default_blending    ::LintPassNonDefaultBlending    >,
    prohibit_compositions:    Option<lint_pass::lint_pass_prohibit_compositions   ::LintPassProhibitCompositions  >,
    prohibit_custom_palettes: Option<lint_pass::lint_pass_prohibit_custom_palettes::LintPassProhibitCustomPalettes>,
    prohibit_kseexpr:         Option<lint_pass::lint_pass_prohibit_kseexpr        ::LintPassProhibitKSeExpr       >,
    software_version:         Option<lint_pass::lint_pass_software_version        ::LintPassSoftwareVersion       >,
    surface_names:            Option<lint_pass::lint_pass_surface_names           ::LintPassSurfaceNames          >,
    surface_type:             Option<lint_pass::lint_pass_surface_type            ::LintPassSurfaceType           >,
    vector_layers:            Option<lint_pass::lint_pass_vector_layers           ::LintPassVectorLayers          >,
}

impl LintPass for LintConfig {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        macro_rules! lint_pass {
            ($lint_name:ident) => {{
                if let Some($lint_name) = self.$lint_name.as_ref() {
                    if let Err(err) = $lint_name.lint(kra_archive, lint_messages) {
                        lint_messages.push("Error", &[meta_error!(err)]);
                    }
                }
            }};
        }

        lint_pass!(animation);
        lint_pass!(colorize_mask);
        lint_pass!(colorspace);
        lint_pass!(copyright);
        lint_pass!(document_name);
        lint_pass!(document_size);
        lint_pass!(document_structure);
        lint_pass!(file_layers);
        lint_pass!(file_name);
        lint_pass!(filters);
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
    pub fn load_from_path(lint_config_path: &Utf8Path) -> Result<LintConfig, LintConfigError> {
        if !lint_config_path.is_file() {
            return Err(LintConfigError::ConfigNotFound { path: lint_config_path.to_owned()});
        }

        let reader = File::open(lint_config_path)
            .map_err(|source| LintConfigError::FailedToOpenConfig { path: lint_config_path.to_owned(), source })?;

        match lint_config_path.extension().map(str::to_lowercase).as_deref() {
            None | Some("toml") => {
                // TODO: toml::from_reader (https://github.com/toml-rs/toml/pull/349)
                toml::from_str(&std::io::read_to_string(reader)?)
                    .map_err(|source| LintConfigError::FailedToParseTomlConfig { path: lint_config_path.to_owned(), source })
            }
            Some("json") => {
                serde_json::from_reader(&reader)
                    .map_err(|source| LintConfigError::FailedToParseJsonConfig { path: lint_config_path.to_owned(), source })
            }
            Some("hjson") => {
                // TODO: deser_hjson::from_reader (https://github.com/Canop/deser-hjson)
                deser_hjson::from_str(&std::io::read_to_string(reader)?)
                    .map_err(|source| LintConfigError::FailedToParseHjsonConfig { path: lint_config_path.to_owned(), source })
            }
            Some("ron") => {
                let ron_options = ron::Options::default()
                    .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);

                ron_options
                    .from_reader(&reader)
                    .map_err(|source| LintConfigError::FailedToParseRonConfig { path: lint_config_path.to_owned(), source })
            }
            Some("yaml" | "yml") => {
                serde_yaml::from_reader(&reader)
                    .map_err(|source| LintConfigError::FailedToParseYamlConfig { path: lint_config_path.to_owned(), source })
            }
            Some("pickle") => {
                let pickle_options = serde_pickle::DeOptions::default();

                serde_pickle::from_reader(reader, pickle_options)
                    .map_err(|source| LintConfigError::FailedToParsePickleConfig { path: lint_config_path.to_owned(), source })
            }
            Some(extension) => {
                Err(LintConfigError::UnknownConfigFormat { extension: extension.to_owned() })
            }
        }
    }

    #[rustfmt::skip]
    pub fn save_to_path(&self, lint_config_path: &Utf8Path) -> Result<(), LintConfigError> {
        let mut writer = File::create(lint_config_path)
            .map_err(|source| LintConfigError::FailedToCreateConfig { path: lint_config_path.to_owned(), source })?;

        match lint_config_path.extension().map(str::to_lowercase).as_deref() {
            None | Some("toml") => {
                // TODO: toml::to_writer (https://github.com/toml-rs/toml/pull/349)
                let tmp_string = toml::ser::to_string_pretty(self)
                    .map_err(LintConfigError::FailedToSerializeTomlConfig)?;
                Ok(writer.write_all(tmp_string.as_bytes())?)
            },
            Some("json") => {
                serde_json::to_writer_pretty(writer, self)
                    .map_err(LintConfigError::FailedToSerializeJsonConfig)
            }
            Some("ron") => {
                let ron_options = ron::Options::default()
                    .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
                let ron_pretty_config = ron::ser::PrettyConfig::default();

                ron_options.to_writer_pretty(writer, self, ron_pretty_config)
                    .map_err(LintConfigError::FailedToSerializeRonConfig)
            }
            Some("yaml" | "yml") => {
                serde_yaml::to_writer(writer, self)
                    .map_err(LintConfigError::FailedToSerializeYamlConfig)
            }
            Some("pickle") => {
                let pickle_options = serde_pickle::SerOptions::default();

                serde_pickle::to_writer(&mut writer, self, pickle_options)
                    .map_err(LintConfigError::FailedToSerializePickleConfig)
            }
            Some(extension) => {
                Err(LintConfigError::UnknownConfigFormat { extension: extension.to_owned() })
            }
        }
    }
}
