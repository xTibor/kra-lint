use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum LintNumberMatchExpression<T>
where
    T: Display,
{
    Value(T),
    LessThan {
        #[serde(rename = "less_than")]
        value: T,
    },
    LessEquals {
        #[serde(rename = "less_equals")]
        value: T,
    },
    GreaterThan {
        #[serde(rename = "greater_than")]
        value: T,
    },
    GreaterEquals {
        #[serde(rename = "greater_equals")]
        value: T,
    },
    Between {
        #[serde(rename = "between")]
        value: (T, T),
    },
    BinaryOr(Vec<LintNumberMatchExpression<T>>),
    BinaryAnd {
        #[serde(rename = "and")]
        expressions: Vec<LintNumberMatchExpression<T>>,
    },
    BinaryNot {
        #[serde(rename = "not")]
        expression: Box<LintNumberMatchExpression<T>>,
    },
}

impl<T> LintNumberMatchExpression<T>
where
    T: PartialEq<T> + PartialOrd<T> + Display,
{
    #[rustfmt::skip]
    pub(crate) fn matches(&self, input: &T) -> bool {
        match self {
            LintNumberMatchExpression::Value(value) => {
                input == value
            }
            LintNumberMatchExpression::LessThan { value } => {
                input < value
            }
            LintNumberMatchExpression::LessEquals { value } => {
                input <= value
            }
            LintNumberMatchExpression::GreaterThan { value } => {
                input > value
            }
            LintNumberMatchExpression::GreaterEquals { value } => {
                input >= value
            }
            LintNumberMatchExpression::Between { value: (value_low, value_high) } => {
                (input >= value_low) && (input <= value_high)
            }
            LintNumberMatchExpression::BinaryOr(expressions) => {
                expressions.iter().any(|expression| expression.matches(input))
            }
            LintNumberMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(input))
            }
            LintNumberMatchExpression::BinaryNot { expression } => {
                !expression.matches(input)
            }
        }
    }
}

impl<T> Display for LintNumberMatchExpression<T>
where
    T: Display,
{
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LintNumberMatchExpression::Value(value) => {
                write!(f, "{}", value)
            }
            LintNumberMatchExpression::LessThan { value } => {
                write!(f, "less_than({})", value)
            }
            LintNumberMatchExpression::LessEquals { value } => {
                write!(f, "less_equals({})", value)
            }
            LintNumberMatchExpression::GreaterThan { value } => {
                write!(f, "greater_than({})", value)
            }
            LintNumberMatchExpression::GreaterEquals { value } => {
                write!(f, "greater_equals({})", value)
            }
            LintNumberMatchExpression::Between { value: (value_low, value_high) } => {
                write!(f, "between({}, {})", value_low, value_high)
            }
            LintNumberMatchExpression::BinaryOr(expressions) => {
                let param_list =
                    expressions.iter().map(LintNumberMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "[{}]", param_list)
            }
            LintNumberMatchExpression::BinaryAnd { expressions } => {
                let param_list =
                    expressions.iter().map(LintNumberMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "and({})", param_list)
            }
            LintNumberMatchExpression::BinaryNot { expression } => {
                write!(f, "not({})", expression)
            }
        }
    }
}
