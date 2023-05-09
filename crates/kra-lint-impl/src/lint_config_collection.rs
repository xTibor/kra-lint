use camino::{Utf8Path, Utf8PathBuf};

use kra_parser::kra_archive::KraArchive;

use crate::{LintConfig, LintPass, LintPassResult};

#[derive(Default)]
pub struct LintConfigCollection {
    pub lint_config_paths: Vec<Utf8PathBuf>,
    pub lint_configs: Vec<LintConfig>,
}

impl LintConfigCollection {
    pub fn load_config(&mut self, lint_config_path: &Utf8Path) {
        let lint_config_path = lint_config_path.to_path_buf();

        // Do not enter infinite loop on circular includes
        if self.lint_config_paths.contains(&lint_config_path) {
            return;
        }

        let lint_config = LintConfig::from_path(&lint_config_path);
        self.lint_config_paths.push(lint_config_path.clone());

        if let Some(lint_includes) = lint_config.includes.as_ref() {
            for include_path in &lint_includes.paths {
                // Include paths are relative to the config file they are defined in
                let resolved_include_path = &lint_config_path
                    .parent()
                    .expect("Failed to get parent directory")
                    .join(include_path);
                self.load_config(resolved_include_path);
            }
        }

        self.lint_configs.push(lint_config);
    }
}

impl LintPass for LintConfigCollection {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        for lint_config in &self.lint_configs {
            results.extend(lint_config.lint(kra_archive));
        }

        results
    }
}
