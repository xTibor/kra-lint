#[derive(Default)]
pub struct LintMessages(Vec<String>);

impl LintMessages {
    pub(crate) fn push(&mut self, lint_message: String) {
        self.0.push(lint_message);
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }
}

impl IntoIterator for LintMessages {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
