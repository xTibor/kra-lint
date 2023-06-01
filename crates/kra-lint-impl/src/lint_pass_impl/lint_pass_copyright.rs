use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config_fields::StringMatchExpression;
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};
use crate::{meta_comment, meta_expected, meta_found, meta_missing_field};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassCopyright {
    copyright_line: Option<StringMatchExpression>,
    copyright_disclaimer: Option<StringMatchExpression>,
    studio_name: Option<StringMatchExpression>,
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
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Missing copyright line",
                        &[
                            meta_expected!(copyright_line),
                        ],
                    );
                } else if !copyright_line.matches(kra_copyright_line) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect copyright line",
                        &[
                            meta_expected!(copyright_line),
                            meta_found!(kra_copyright_line),
                        ],
                    );
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(copyright_disclaimer) = self.copyright_disclaimer.as_ref() {
                let kra_copyright_disclaimer = &kra_archive.document_info.about.r#abstract;

                if kra_copyright_disclaimer.is_empty() {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Missing copyright disclaimer",
                        &[
                            meta_expected!(copyright_disclaimer),
                        ],
                    );
                } else if !copyright_disclaimer.matches(kra_copyright_disclaimer) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect copyright disclaimer",
                        &[
                            meta_expected!(copyright_disclaimer),
                            meta_found!(kra_copyright_disclaimer),
                        ],
                    );
                }
            }
        }

        // Sub-pass #3
        {
            if self.ensure_initial_author_exists == Some(true) {
                let kra_initial_creator = &kra_archive.document_info.about.initial_creator;

                if kra_initial_creator.is_empty() || (kra_initial_creator == "Unknown") {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Missing author information",
                        &[
                            meta_missing_field!("Initial creator"),
                        ],
                    );
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
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Missing author information",
                        &[
                            meta_missing_field!("Author full name"),
                        ],
                    );
                }

                if kra_author_first_name.is_empty() {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Missing author information",
                        &[
                            meta_missing_field!("Author first name"),
                        ],
                    );
                }

                if kra_author_last_name.is_empty() {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Missing author information",
                        &[
                            meta_missing_field!("Author last name"),
                        ],
                    );
                }

                if !kra_author_full_name.is_empty() && !kra_author_first_name.is_empty() {
                    // .contains() because Eastern/Western name orders
                    if !kra_author_full_name.contains(kra_author_first_name) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Inconsistent author information",
                            &[
                                meta_comment!(format!("First name: \"{}\"", kra_author_first_name.to_string())),
                                meta_comment!(format!("Full name: \"{}\"", kra_author_full_name.to_string())),
                            ],
                        );
                    }
                }

                if !kra_author_full_name.is_empty() && !kra_author_last_name.is_empty() {
                    // .contains() because Eastern/Western name orders
                    if !kra_author_full_name.contains(kra_author_last_name) {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Inconsistent author information",
                            &[
                                meta_comment!(format!("Last name: \"{}\"", kra_author_last_name.to_string())),
                                meta_comment!(format!("Full name: \"{}\"", kra_author_full_name.to_string())),
                            ],
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
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Missing studio name",
                        &[
                            meta_expected!(studio_name),
                        ],
                    );
                } else if !studio_name.matches(kra_studio_name) {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect studio name",
                        &[
                            meta_expected!(studio_name),
                            meta_found!(kra_studio_name),
                        ],
                    );
                }
            }
        }

        Ok(())
    }
}
