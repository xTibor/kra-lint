use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum LintStringMatchExpression {
    FullMatch(String),
    Regex {
        #[serde(rename = "regex")]
        pattern: String,
    },
    StartsWith {
        #[serde(rename = "starts_with")]
        pattern: String,
    },
    EndsWith {
        #[serde(rename = "ends_with")]
        pattern: String,
    },
    Contains {
        #[serde(rename = "contains")]
        pattern: String,
    },
    BinaryOr(Vec<LintStringMatchExpression>),
    BinaryAnd {
        #[serde(rename = "and")]
        expressions: Vec<LintStringMatchExpression>,
    },
    BinaryNot {
        #[serde(rename = "not")]
        expression: Box<LintStringMatchExpression>,
    },
}

impl LintStringMatchExpression {
    #[rustfmt::skip]
    pub(crate) fn matches(&self, input: &str) -> bool {
        match self {
            LintStringMatchExpression::FullMatch(pattern) => {
                input == pattern
            }
            LintStringMatchExpression::Regex { pattern } => {
                let compiled_regex = regex::Regex::new(pattern)
                    .expect("Failed to compile regular expression");
                compiled_regex.is_match(input)
            }
            LintStringMatchExpression::StartsWith { pattern } => {
                input.starts_with(pattern)
            }
            LintStringMatchExpression::EndsWith { pattern } => {
                input.ends_with(pattern)
            }
            LintStringMatchExpression::Contains { pattern } => {
                input.contains(pattern)
            }
            LintStringMatchExpression::BinaryOr(expressions) => {
                expressions.iter().any(|expression| expression.matches(input))
            }
            LintStringMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(input))
            }
            LintStringMatchExpression::BinaryNot { expression } => {
                !expression.matches(input)
            }
        }
    }
}

impl std::fmt::Display for LintStringMatchExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LintStringMatchExpression::FullMatch(pattern) => {
                write!(f, "\"{}\"", pattern)
            }
            LintStringMatchExpression::Regex { pattern } => {
                write!(f, "regex(\"{}\")", pattern)
            }
            LintStringMatchExpression::StartsWith { pattern } => {
                write!(f, "starts_with(\"{}\")", pattern)
            }
            LintStringMatchExpression::EndsWith { pattern } => {
                write!(f, "ends_with(\"{}\")", pattern)
            }
            LintStringMatchExpression::Contains { pattern } => {
                write!(f, "contains(\"{}\")", pattern)
            }
            LintStringMatchExpression::BinaryOr(expressions) => {
                let param_list =
                    expressions.iter().map(LintStringMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "[{}]", param_list)
            }
            LintStringMatchExpression::BinaryAnd { expressions } => {
                let param_list =
                    expressions.iter().map(LintStringMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "and({})", param_list)
            }
            LintStringMatchExpression::BinaryNot { expression } => {
                write!(f, "not({})", expression)
            }
        }
    }
}
