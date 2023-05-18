#[derive(Default)]
pub struct LintMessages(Vec<(String, String)>);

impl LintMessages {
    pub(crate) fn push<S1, S2>(&mut self, lint_title: S1, lint_message: S2)
    where
        S1: AsRef<str> + Into<String>,
        S2: AsRef<str> + Into<String>,
    {
        self.0.push((lint_title.into(), lint_message.into()));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, String)> {
        self.0.iter()
    }
}

impl IntoIterator for LintMessages {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
