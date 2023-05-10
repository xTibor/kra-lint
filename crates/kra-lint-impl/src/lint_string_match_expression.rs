use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum LintStringMatchExpression {
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
        expressions: Vec<LintStringMatchExpression>,
    },
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
    pub fn matches(&self, value: &str) -> bool {
        match self {
            LintStringMatchExpression::FullMatch(pattern) => value == pattern,
            LintStringMatchExpression::Regex { pattern } => {
                let compiled_regex = regex::Regex::new(pattern).expect("Failed to compile regular expression");
                compiled_regex.is_match(value)
            }
            LintStringMatchExpression::StartsWith { pattern } => value.starts_with(pattern),
            LintStringMatchExpression::EndsWith { pattern } => value.ends_with(pattern),
            LintStringMatchExpression::Contains { pattern } => value.contains(pattern),
            LintStringMatchExpression::BinaryOr { expressions } => {
                expressions.iter().any(|expression| expression.matches(value))
            }
            LintStringMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(value))
            }
            LintStringMatchExpression::BinaryNot { expression } => !expression.matches(value),
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
            LintStringMatchExpression::BinaryOr { expressions } => {
                let param_list =
                    expressions.iter().map(LintStringMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "or({})", param_list)
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
