use strum::EnumString;

#[non_exhaustive]
#[derive(EnumString, Copy, Clone, Debug)]
#[strum(ascii_case_insensitive)]
pub enum LintOutputFormat {
    #[strum(serialize = "plain-text")]
    PlainText,

    #[strum(serialize = "json")]
    Json,

    #[strum(serialize = "ron")]
    Ron,

    #[strum(serialize = "yaml")]
    Yaml,

    #[strum(serialize = "pickle")]
    Pickle,
}
