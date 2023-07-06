use strum::EnumString;

#[non_exhaustive]
#[derive(EnumString, Copy, Clone, Debug)]
#[strum(ascii_case_insensitive)]
pub enum LintOutputFormat {
    #[cfg(feature = "output-plaintext")]
    #[strum(serialize = "plain-text")]
    PlainText,

    #[cfg(feature = "output-json")]
    #[strum(serialize = "json")]
    Json,

    #[cfg(feature = "output-ron")]
    #[strum(serialize = "ron")]
    Ron,

    #[cfg(feature = "output-yaml")]
    #[strum(serialize = "yaml")]
    Yaml,

    #[cfg(feature = "output-pickle")]
    #[strum(serialize = "pickle")]
    Pickle,

    #[cfg(feature = "output-gura")]
    #[strum(serialize = "gura")]
    Gura,
}
