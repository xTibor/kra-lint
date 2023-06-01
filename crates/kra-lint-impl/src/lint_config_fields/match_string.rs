use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum StringMatchExpression {
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
    BinaryOr(Vec<StringMatchExpression>),
    BinaryAnd {
        #[serde(rename = "and")]
        expressions: Vec<StringMatchExpression>,
    },
    BinaryNot {
        #[serde(rename = "not")]
        expression: Box<StringMatchExpression>,
    },
}

impl StringMatchExpression {
    #[rustfmt::skip]
    pub(crate) fn matches(&self, input: &str) -> bool {
        match self {
            StringMatchExpression::FullMatch(pattern) => {
                input == pattern
            }
            StringMatchExpression::Regex { pattern } => {
                let compiled_regex = regex::Regex::new(pattern)
                    .expect("Failed to compile regular expression");
                compiled_regex.is_match(input)
            }
            StringMatchExpression::StartsWith { pattern } => {
                input.starts_with(pattern)
            }
            StringMatchExpression::EndsWith { pattern } => {
                input.ends_with(pattern)
            }
            StringMatchExpression::Contains { pattern } => {
                input.contains(pattern)
            }
            StringMatchExpression::BinaryOr(expressions) => {
                expressions.iter().any(|expression| expression.matches(input))
            }
            StringMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(input))
            }
            StringMatchExpression::BinaryNot { expression } => {
                !expression.matches(input)
            }
        }
    }
}

impl Display for StringMatchExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StringMatchExpression::FullMatch(pattern) => {
                write!(f, "\"{}\"", pattern)
            }
            StringMatchExpression::Regex { pattern } => {
                write!(f, "regex(\"{}\")", pattern)
            }
            StringMatchExpression::StartsWith { pattern } => {
                write!(f, "starts_with(\"{}\")", pattern)
            }
            StringMatchExpression::EndsWith { pattern } => {
                write!(f, "ends_with(\"{}\")", pattern)
            }
            StringMatchExpression::Contains { pattern } => {
                write!(f, "contains(\"{}\")", pattern)
            }
            StringMatchExpression::BinaryOr(expressions) => {
                let param_list =
                    expressions.iter().map(StringMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "[{}]", param_list)
            }
            StringMatchExpression::BinaryAnd { expressions } => {
                let param_list =
                    expressions.iter().map(StringMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "and({})", param_list)
            }
            StringMatchExpression::BinaryNot { expression } => {
                write!(f, "not({})", expression)
            }
        }
    }
}
