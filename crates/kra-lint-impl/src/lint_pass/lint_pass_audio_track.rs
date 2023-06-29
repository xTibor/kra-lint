use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use kra_parser::kra_archive::KraArchive;

use crate::lint_config_fields::StringMatchExpression;
use crate::lint_output::lint_metadata_macros::{meta_comment, meta_expected, meta_found};
use crate::lint_output::LintMessages;
use crate::lint_pass::{LintPass, LintPassResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LintPassAudioTrack {
    audio_track_present: Option<bool>,
    file_formats: Option<StringMatchExpression>,
    check_missing_files: Option<bool>,
    default_volume: Option<bool>,
}

impl LintPass for LintPassAudioTrack {
    fn lint(&self, kra_archive: &KraArchive, lint_messages: &mut LintMessages) -> LintPassResult {
        // Sub-pass #1
        {
            if let Some(audio_track_present) = self.audio_track_present {
                let kra_audio_track_present = kra_archive.main_doc.image.audio.is_some();

                if audio_track_present && !kra_audio_track_present {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect audio track",
                        &[
                            meta_comment!("Required audio track not present"),
                        ],
                    );
                } else if !audio_track_present && kra_audio_track_present {
                    #[rustfmt::skip]
                    lint_messages.push(
                        "Incorrect audio track",
                        &[
                            meta_comment!("Unexpected audio track present"),
                        ],
                    );
                }
            }
        }

        // Sub-pass #2
        {
            if let Some(file_formats) = self.file_formats.as_ref() {
                // Bug: Changing audio tracks do not set the modified flag on the document.
                //   Cannot save audio track changes by themselves without other document changes.
                if let Some(kra_audio_track) = kra_archive.main_doc.image.audio.as_ref() {
                    let kra_audio_path: &str = kra_audio_track.master_channel_path.value.as_ref();

                    if let Some(source_ext) = Utf8Path::new(&kra_audio_path).extension() {
                        if !file_formats.matches(source_ext) {
                            #[rustfmt::skip]
                            lint_messages.push(
                                "Incorrect audio track format",
                                &[
                                    meta_expected!(file_formats),
                                    meta_found!(source_ext),
                                ],
                            );
                        }
                    } else {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Audio track file has no file extension",
                            &[
                                meta_expected!(file_formats),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #3
        {
            if self.check_missing_files == Some(true) {
                if let Some(kra_audio_track) = kra_archive.main_doc.image.audio.as_ref() {
                    // Audio path field stores relative paths, path traversal is intentional here.
                    let resolved_source_path = kra_archive
                        .zip_path
                        .parent()
                        .expect("Failed to get document parent directory")
                        .join(&kra_audio_track.master_channel_path.value);

                    if !resolved_source_path.is_file() {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Missing audio track file",
                            &[
                                meta_comment!(format!("Source: \"{}\"", resolved_source_path)),
                            ],
                        );
                    }
                }
            }
        }

        // Sub-pass #4
        {
            if self.default_volume == Some(true) {
                if let Some(kra_audio_track) = kra_archive.main_doc.image.audio.as_ref() {
                    let kra_audio_muted = kra_audio_track.audio_muted.value;
                    let kra_audio_volume = kra_audio_track.audio_volume.value;

                    if kra_audio_muted == 1 {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Non default audio track volume",
                            &[
                                meta_comment!("Muted audio track"),
                            ],
                        );
                    }

                    if kra_audio_volume != 0.5 {
                        #[rustfmt::skip]
                        lint_messages.push(
                            "Non default audio track volume",
                            &[
                                meta_expected!("50%"),
                                meta_found!(format!("{:.0}%", kra_audio_volume * 100.0)),
                            ],
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
