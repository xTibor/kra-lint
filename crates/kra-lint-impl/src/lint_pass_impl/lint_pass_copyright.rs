use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_fields::LintStringMatchExpression;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::LintMessages;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassCopyright {
    copyright_line: Option<LintStringMatchExpression>,
    copyright_disclaimer: Option<LintStringMatchExpression>,
    studio_name: Option<LintStringMatchExpression>,
    ensure_initial_author_exists: Option<bool>,
    ensure_author_exists: Option<bool>,
}

impl LintPass for LintPassCopyright {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(copyright_line) = self.copyright_line.as_ref() {
                let kra_copyright_line = &kra_archive.document_info.about.license;

                if kra_copyright_line.is_empty() {
                    lint_messages.push("Missing copyright line", format!("Expected: {}", copyright_line));
                } else if !copyright_line.matches(kra_copyright_line) {
                    lint_messages.push(
                        "Incorrect copyright line",
                        format!("Expected: {}, Found: \"{}\"", copyright_line, kra_copyright_line.escape_debug()),
                    );
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(copyright_disclaimer) = self.copyright_disclaimer.as_ref() {
                let kra_copyright_disclaimer = &kra_archive.document_info.about.r#abstract;

                if kra_copyright_disclaimer.is_empty() {
                    lint_messages.push("Missing copyright disclaimer", format!("Expected: {}", copyright_disclaimer));
                } else if !copyright_disclaimer.matches(kra_copyright_disclaimer) {
                    lint_messages.push(
                        "Incorrect copyright disclaimer",
                        format!(
                            "Expected: {}, Found: \"{}\"",
                            copyright_disclaimer,
                            kra_copyright_disclaimer.escape_debug()
                        ),
                    );
                }
            }
        }

        // Sub-pass #3
        {
            if self.ensure_initial_author_exists == Some(true) {
                let kra_initial_creator = &kra_archive.document_info.about.initial_creator;

                if kra_initial_creator.is_empty() || (kra_initial_creator == "Unknown") {
                    lint_messages.push("Missing author information", "Initial creator");
                }
            }
        }

        // Sub-pass #4
        {
            if self.ensure_author_exists == Some(true) {
                let kra_author_full_name = &kra_archive.document_info.author.full_name;
                let kra_author_first_name = &kra_archive.document_info.author.creator_first_name;
                let kra_author_last_name = &kra_archive.document_info.author.creator_last_name;

                if kra_author_full_name.is_empty() {
                    lint_messages.push("Missing author information", "Author full name");
                }

                if kra_author_first_name.is_empty() {
                    lint_messages.push("Missing author information", "Author first name");
                }

                if kra_author_last_name.is_empty() {
                    lint_messages.push("Missing author information", "Author last name");
                }

                if !kra_author_full_name.is_empty() && !kra_author_first_name.is_empty() {
                    // .contains() because Eastern/Western name orders
                    if !kra_author_full_name.contains(kra_author_first_name) {
                        lint_messages.push(
                            "Inconsistent author information",
                            format!(
                                "First name: \"{}\", Full name: \"{}\"",
                                kra_author_first_name.escape_debug(),
                                kra_author_full_name.escape_debug()
                            ),
                        );
                    }
                }

                if !kra_author_full_name.is_empty() && !kra_author_last_name.is_empty() {
                    // .contains() because Eastern/Western name orders
                    if !kra_author_full_name.contains(kra_author_last_name) {
                        lint_messages.push(
                            "Inconsistent author information",
                            format!(
                                "Last name: \"{}\", Full name: \"{}\"",
                                kra_author_last_name.escape_debug(),
                                kra_author_full_name.escape_debug()
                            ),
                        );
                    }
                }
            }
        }

        // Sub-pass #5
        {
            if let Some(studio_name) = self.studio_name.as_ref() {
                let kra_studio_name = &kra_archive.document_info.author.company;

                if kra_studio_name.is_empty() {
                    lint_messages.push("Missing studio name", format!("Expected: {}", studio_name));
                } else if !studio_name.matches(kra_studio_name) {
                    lint_messages.push(
                        "Incorrect studio name",
                        format!("Expected: {}, Found: \"{}\"", studio_name, kra_studio_name.escape_debug()),
                    );
                }
            }
        }

        Ok(())
    }
}
