use camino::{Utf8Path, Utf8PathBuf};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config::lint_config::LintConfig;
use crate::lint_config::lint_config_error::LintConfigError;
use crate::lint_output::{LintMessages, LintMessagesCollection};
use crate::lint_pass::{LintPass, LintPassResult};
use crate::meta_error;

#[derive(Default)]
pub struct LintConfigCollection {
    lint_config_paths: Vec<Utf8PathBuf>,
    lint_configs: Vec<LintConfig>,
}

impl LintConfigCollection {
    pub fn load_config(&mut self, lint_config_path: &Utf8Path) -> Result<(), LintConfigError> {
        if !lint_config_path.is_file() {
            return Err(LintConfigError::ConfigNotFound { path: lint_config_path.to_owned() });
        }

        let lint_config_path = lint_config_path.canonicalize_utf8()?;

        // Do not enter infinite loop on circular includes
        if self.lint_config_paths.contains(&lint_config_path) {
            return Ok(());
        }

        let lint_config = LintConfig::load_from_path(&lint_config_path)?;
        self.lint_config_paths.push(lint_config_path.clone());

        if let Some(lint_includes) = lint_config.includes.as_ref() {
            for include_path in &lint_includes.paths {
                if include_path.is_absolute() {
                    if !include_path.is_file() {
                        return Err(LintConfigError::ConfigIncludeNotFound {
                            path: include_path.to_owned(),
                            included_from: lint_config_path,
                        });
                    }

                    self.load_config(include_path)?;
                } else {
                    // Relative paths are relative to the config file they are defined in
                    let resolved_include_path =
                        lint_config_path.parent().expect("Failed to get parent directory").join(include_path);

                    if !resolved_include_path.is_file() {
                        return Err(LintConfigError::ConfigIncludeNotFound {
                            path: resolved_include_path,
                            included_from: lint_config_path,
                        });
                    }

                    let resolved_include_path = resolved_include_path.canonicalize_utf8()?;
                    self.load_config(&resolved_include_path)?;
                }
            }
        }

        self.lint_configs.push(lint_config);
        Ok(())
    }

    pub fn lint_path<P>(&self, kra_path: &P) -> LintMessages
    where
        P: AsRef<Utf8Path> + Into<Utf8PathBuf>,
    {
        let mut lint_messages = LintMessages::default();

        match KraArchive::from_path(kra_path.as_ref()) {
            Ok(kra_archive) => match self.lint(&kra_archive, &mut lint_messages) {
                Ok(()) => {}
                Err(err) => lint_messages.push("Error", &[meta_error!(err)]),
            },
            Err(err) => lint_messages.push("Error", &[meta_error!(err)]),
        }

        lint_messages.sort_and_dedup();
        lint_messages
    }

    pub fn lint_paths<P>(&self, kra_paths: &[P]) -> LintMessagesCollection
    where
        P: AsRef<Utf8Path> + Into<Utf8PathBuf>,
    {
        let mut lint_message_collection = LintMessagesCollection::default();

        for kra_path in kra_paths {
            let lint_messages = self.lint_path(kra_path);

            if !lint_messages.is_empty() {
                lint_message_collection.push(kra_path.as_ref(), lint_messages);
            }
        }

        lint_message_collection
    }
}

impl LintPass for LintConfigCollection {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        for lint_config in &self.lint_configs {
            lint_config.lint(kra_archive, lint_messages)?;
        }

        Ok(())
    }
}
