use camino::{Utf8Path, Utf8PathBuf};

use kra_parser::kra_archive::KraArchive;

use crate::{LintConfig, LintError, LintPass, LintPassResult};

#[derive(Default)]
pub struct LintConfigCollection {
    pub lint_config_paths: Vec<Utf8PathBuf>,
    pub lint_configs: Vec<LintConfig>,
}

impl LintConfigCollection {
    pub fn load_config(
        &mut self,
        lint_config_path: &Utf8Path,
    ) -> Result<(), LintError> {
        if !lint_config_path.is_file() {
            return Err(LintError::ConfigNotFound(lint_config_path.to_owned()));
        }

        let lint_config_path = lint_config_path
            .canonicalize_utf8()
            .expect("Failed to canonicalize path");

        // Do not enter infinite loop on circular includes
        if self.lint_config_paths.contains(&lint_config_path) {
            return Ok(());
        }

        let lint_config = LintConfig::from_path(&lint_config_path)?;
        self.lint_config_paths.push(lint_config_path.clone());

        if let Some(lint_includes) = lint_config.includes.as_ref() {
            for include_path in &lint_includes.paths {
                if include_path.is_absolute() {
                    if !include_path.is_file() {
                        return Err(LintError::ConfigIncludeNotFound(
                            include_path.to_owned(),
                            lint_config_path.to_owned(),
                        ));
                    }

                    self.load_config(include_path)?;
                } else {
                    // Relative paths are relative to the config file they are defined in
                    let resolved_include_path = lint_config_path
                        .parent()
                        .expect("Failed to get parent directory")
                        .join(include_path);

                    if !resolved_include_path.is_file() {
                        return Err(LintError::ConfigIncludeNotFound(
                            resolved_include_path.to_owned(),
                            lint_config_path.to_owned(),
                        ));
                    }

                    let resolved_include_path = resolved_include_path
                        .canonicalize_utf8()
                        .expect("Failed to canonicalize path");

                    self.load_config(&resolved_include_path)?;
                }
            }
        }

        self.lint_configs.push(lint_config);
        Ok(())
    }
}

impl LintPass for LintConfigCollection {
    fn lint(
        &self,
        kra_archive: &KraArchive,
        lint_messages: &mut Vec<String>,
    ) -> LintPassResult {
        for lint_config in &self.lint_configs {
            lint_config.lint(kra_archive, lint_messages)?;
        }

        Ok(())
    }
}
