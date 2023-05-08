use kra_parser::kra_archive::KraArchive;

pub type LintPassResult = Vec<String>;

pub trait LintPass {
    fn lint(&self, kra_archive: &KraArchive) -> LintPassResult;
}
