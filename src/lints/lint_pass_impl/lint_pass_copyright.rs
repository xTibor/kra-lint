use serde::Deserialize;

use crate::lints::{LintPass, LintPassResult, LintStringMatchExpression};
use crate::models::kra_archive::KraArchive;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintPassCopyright {
    pub copyright_line: Option<LintStringMatchExpression>,
    pub copyright_disclaimer: Option<LintStringMatchExpression>,
    pub studio_name: Option<LintStringMatchExpression>,
    pub ensure_initial_author_exists: Option<bool>,
    pub ensure_author_exists: Option<bool>,
}

impl LintPass for LintPassCopyright {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult {
        let mut results = vec![];

        // Sub-pass #1
        {
            if let Some(copyright_line) = self.copyright_line.as_ref() {
                let kra_copyright_line =
                    &kra_archive.document_info.about.license;

                if kra_copyright_line.is_empty() {
                    results.push("Missing copyright line".to_owned());
                } else if !copyright_line.matches(kra_copyright_line) {
                    results.push(format!(
                    "Incorrect copyright line (expected: {}, found: \"{}\")",
                    copyright_line, kra_copyright_line,
                ));
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(copyright_disclaimer) =
                self.copyright_disclaimer.as_ref()
            {
                let kra_copyright_disclaimer =
                    &kra_archive.document_info.about.r#abstract;

                if kra_copyright_disclaimer.is_empty() {
                    results.push("Missing copyright disclaimer".to_owned());
                } else if !copyright_disclaimer
                    .matches(kra_copyright_disclaimer)
                {
                    results.push(format!(
                    "Incorrect copyright disclaimer (expected: {}, found: \"{}\")",
                    copyright_disclaimer, kra_copyright_disclaimer,
                ));
                }
            }
        }

        // Sub-pass #3
        {
            if self.ensure_initial_author_exists == Some(true) {
                let kra_initial_creator =
                    &kra_archive.document_info.about.initial_creator;

                if kra_initial_creator.is_empty()
                    || (kra_initial_creator == "Unknown")
                {
                    results.push(
                        "Missing author information (Initial creator)"
                            .to_owned(),
                    );
                }
            }
        }

        // Sub-pass #4
        {
            if self.ensure_author_exists == Some(true) {
                let kra_author_full_name =
                    &kra_archive.document_info.author.full_name;

                let kra_author_first_name =
                    &kra_archive.document_info.author.creator_first_name;

                let kra_author_last_name =
                    &kra_archive.document_info.author.creator_last_name;

                if kra_author_full_name.is_empty() {
                    results.push(
                        "Missing author information (Author full name)"
                            .to_owned(),
                    );
                }

                if kra_author_first_name.is_empty() {
                    results.push(
                        "Missing author information (Author first name)"
                            .to_owned(),
                    );
                }

                if kra_author_last_name.is_empty() {
                    results.push(
                        "Missing author information (Author last name)"
                            .to_owned(),
                    );
                }

                if !kra_author_full_name.is_empty()
                    && !kra_author_first_name.is_empty()
                {
                    // .contains() because Eastern/Western name orders
                    if !kra_author_full_name.contains(kra_author_first_name) {
                        results.push(format!(
                            "Inconsistent author information (first name: \"{}\", full name: \"{}\")",
                            kra_author_first_name, kra_author_full_name
                        ));
                    }
                }

                if !kra_author_full_name.is_empty()
                    && !kra_author_last_name.is_empty()
                {
                    // .contains() because Eastern/Western name orders
                    if !kra_author_full_name.contains(kra_author_last_name) {
                        results.push(format!(
                            "Inconsistent author information (last name: \"{}\", full name: \"{}\")",
                            kra_author_last_name, kra_author_full_name
                        ));
                    }
                }
            }
        }

        // Sub-pass #5
        {
            if let Some(studio_name) = self.studio_name.as_ref() {
                let kra_studio_name = &kra_archive.document_info.author.company;

                if kra_studio_name.is_empty() {
                    results.push("Missing studio name".to_owned());
                } else if !studio_name.matches(kra_studio_name) {
                    results.push(format!(
                        "Incorrect studio name (expected: {}, found: \"{}\")",
                        studio_name, kra_studio_name,
                    ));
                }
            }
        }

        results
    }
}
