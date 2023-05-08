use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum StringMatchExpression {
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
    BinaryOr {
        #[serde(rename = "or")]
        expressions: Vec<StringMatchExpression>,
    },
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
    pub fn matches(&self, value: &str) -> bool {
        match self {
            StringMatchExpression::FullMatch(pattern) => value == pattern,
            StringMatchExpression::Regex { pattern } => {
                let compiled_regex = regex::Regex::new(pattern)
                    .expect("Failed to compile regular expression");
                compiled_regex.is_match(value)
            }
            StringMatchExpression::StartsWith { pattern } => {
                value.starts_with(pattern)
            }
            StringMatchExpression::EndsWith { pattern } => {
                value.ends_with(pattern)
            }
            StringMatchExpression::Contains { pattern } => {
                value.contains(pattern)
            }
            StringMatchExpression::BinaryOr { expressions } => {
                expressions.iter().any(|expression| expression.matches(value))
            }
            StringMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(value))
            }
            StringMatchExpression::BinaryNot { expression } => {
                !expression.matches(value)
            }
        }
    }
}

impl std::fmt::Display for StringMatchExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            StringMatchExpression::BinaryOr { expressions } => {
                let param_list = expressions
                    .iter()
                    .map(StringMatchExpression::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "or({})", param_list)
            }
            StringMatchExpression::BinaryAnd { expressions } => {
                let param_list = expressions
                    .iter()
                    .map(StringMatchExpression::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "and({})", param_list)
            }
            StringMatchExpression::BinaryNot { expression } => {
                write!(f, "not({})", expression)
            }
        }
    }
}
