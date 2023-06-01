mod lint_pass_error;
mod lint_pass;

pub(crate) use lint_pass::{LintPass, LintPassResult};
pub use lint_pass_error::LintPassError;
