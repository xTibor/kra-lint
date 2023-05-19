use strum::EnumString;

#[derive(EnumString, Copy, Clone, Debug)]
pub enum LintOutputFormat {
    #[strum(serialize = "plain-text")]
    PlainText,

    #[strum(serialize = "toml")]
    Toml,

    #[strum(serialize = "json")]
    Json,

    #[strum(serialize = "ron")]
    Ron,

    #[strum(serialize = "yaml")]
    Yaml,

    #[strum(serialize = "pickle")]
    Pickle,
}
