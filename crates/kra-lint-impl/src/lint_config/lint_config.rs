use std::fs::File;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config::LintConfigError;
use crate::lint_output::lint_metadata_macros::meta_error;
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
    audio_track:              Option<lint_pass::lint_pass_audio_track             ::LintPassAudioTrack            >,
    colorize_mask:            Option<lint_pass::lint_pass_colorize_mask           ::LintPassColorizeMask          >,
    colorspace:               Option<lint_pass::lint_pass_colorspace              ::LintPassColorspace            >,
    copyright:                Option<lint_pass::lint_pass_copyright               ::LintPassCopyright             >,
    document_name:            Option<lint_pass::lint_pass_document_name           ::LintPassDocumentName          >,
    document_size:            Option<lint_pass::lint_pass_document_size           ::LintPassDocumentSize          >,
    document_structure:       Option<lint_pass::lint_pass_document_structure      ::LintPassDocumentStructure     >,
    file_layers:              Option<lint_pass::lint_pass_file_layers             ::LintPassFileLayers            >,
    file_name:                Option<lint_pass::lint_pass_file_name               ::LintPassFileName              >,
    file_permissions:         Option<lint_pass::lint_pass_file_permissions        ::LintPassFilePermissions       >,
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
        lint_pass!(audio_track);
        lint_pass!(colorize_mask);
        lint_pass!(colorspace);
        lint_pass!(copyright);
        lint_pass!(document_name);
        lint_pass!(document_size);
        lint_pass!(document_structure);
        lint_pass!(file_layers);
        lint_pass!(file_name);
        lint_pass!(file_permissions);
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
            return Err(LintConfigError::ConfigNotFound { path: lint_config_path.into()});
        }

        #[allow(unused_variables)]
        let reader = File::open(lint_config_path)
            .map_err(|source| LintConfigError::FailedToOpenConfig { path: lint_config_path.into(), source })?;

        let lint_config_extension = lint_config_path.extension().unwrap_or("toml").to_lowercase();

        match lint_config_extension.as_str() {
            #[cfg(feature = "config-toml")]
            "toml" => {
                toml_ext::from_reader(reader)
                    .map_err(|source| LintConfigError::FailedToParseTomlConfig { path: lint_config_path.into(), source })
            }

            #[cfg(feature = "config-json")]
            "json" => {
                serde_json::from_reader(reader)
                    .map_err(|source| LintConfigError::FailedToParseJsonConfig { path: lint_config_path.into(), source })
            }

            #[cfg(feature = "config-hjson")]
            "hjson" => {
                deser_hjson::from_reader(reader)
                    .map_err(|source| LintConfigError::FailedToParseHjsonConfig { path: lint_config_path.into(), source })
            }

            #[cfg(feature = "config-ron")]
            "ron" => {
                let ron_options = ron::Options::default()
                    .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);

                ron_options
                    .from_reader(reader)
                    .map_err(|source| LintConfigError::FailedToParseRonConfig { path: lint_config_path.into(), source })
            }

            #[cfg(feature = "config-yaml")]
            "yaml" | "yml" => {
                serde_yaml::from_reader(reader)
                    .map_err(|source| LintConfigError::FailedToParseYamlConfig { path: lint_config_path.into(), source })
            }

            #[cfg(feature = "config-pickle")]
            "pickle" => {
                let pickle_options = serde_pickle::DeOptions::default();

                serde_pickle::from_reader(reader, pickle_options)
                    .map_err(|source| LintConfigError::FailedToParsePickleConfig { path: lint_config_path.into(), source })
            }

            #[cfg(feature = "config-gura")]
            "gura" | "ura" => {
                gura_ext::from_reader(reader)
                    .map_err(|source| LintConfigError::FailedToParseGuraConfig { path: lint_config_path.into(), source })
            }

            extension => {
                Err(LintConfigError::UnknownConfigFormat { path: lint_config_path.into(), extension: extension.to_owned() })
            }
        }
    }

    #[rustfmt::skip]
    pub fn save_to_path(&self, lint_config_path: &Utf8Path) -> Result<(), LintConfigError> {
        #[allow(unused_variables, unused_mut)]
        let mut writer = File::create(lint_config_path)
            .map_err(|source| LintConfigError::FailedToCreateConfig { path: lint_config_path.into(), source })?;

        let lint_config_extension = lint_config_path.extension().unwrap_or("toml").to_lowercase();

        match lint_config_extension.as_str() {
            #[cfg(feature = "config-toml")]
            "toml" => {
                toml_ext::to_writer(writer, self)
                    .map_err(LintConfigError::FailedToSerializeTomlConfig)
            }

            #[cfg(feature = "config-json")]
            "json" => {
                serde_json::to_writer_pretty(writer, self)
                    .map_err(LintConfigError::FailedToSerializeJsonConfig)
            }

            #[cfg(feature = "config-ron")]
            "ron" => {
                let ron_options = ron::Options::default()
                    .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
                let ron_pretty_config = ron::ser::PrettyConfig::default();

                ron_options.to_writer_pretty(writer, self, ron_pretty_config)
                    .map_err(LintConfigError::FailedToSerializeRonConfig)
            }

            #[cfg(feature = "config-yaml")]
            "yaml" | "yml" => {
                serde_yaml::to_writer(writer, self)
                    .map_err(LintConfigError::FailedToSerializeYamlConfig)
            }

            #[cfg(feature = "config-pickle")]
            "pickle" => {
                let pickle_options = serde_pickle::SerOptions::default();

                serde_pickle::to_writer(&mut writer, self, pickle_options)
                    .map_err(LintConfigError::FailedToSerializePickleConfig)
            }

            #[cfg(feature = "config-gura")]
            "gura" | "ura" => {
                gura_ext::to_writer(writer, self)
                    .map_err(LintConfigError::FailedToSerializeGuraConfig)
            }

            extension => {
                Err(LintConfigError::UnknownConfigFormat { path: lint_config_path.into(), extension: extension.to_owned() })
            }
        }
    }
}
