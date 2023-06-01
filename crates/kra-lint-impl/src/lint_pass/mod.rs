mod lint_pass;
mod lint_pass_error;

pub(crate) use lint_pass::{LintPass, LintPassResult};
pub use lint_pass_error::LintPassError;
