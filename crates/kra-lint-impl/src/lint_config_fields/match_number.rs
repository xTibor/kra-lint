use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum NumberMatchExpression<T>
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
    BinaryOr(Vec<NumberMatchExpression<T>>),
    BinaryAnd {
        #[serde(rename = "and")]
        expressions: Vec<NumberMatchExpression<T>>,
    },
    BinaryNot {
        #[serde(rename = "not")]
        expression: Box<NumberMatchExpression<T>>,
    },
}

impl<T> NumberMatchExpression<T>
where
    T: PartialEq<T> + PartialOrd<T> + Display,
{
    #[rustfmt::skip]
    pub(crate) fn matches(&self, input: &T) -> bool {
        match self {
            NumberMatchExpression::Value(value) => {
                input == value
            }
            NumberMatchExpression::LessThan { value } => {
                input < value
            }
            NumberMatchExpression::LessEquals { value } => {
                input <= value
            }
            NumberMatchExpression::GreaterThan { value } => {
                input > value
            }
            NumberMatchExpression::GreaterEquals { value } => {
                input >= value
            }
            NumberMatchExpression::Between { value: (value_low, value_high) } => {
                (input >= value_low) && (input <= value_high)
            }
            NumberMatchExpression::BinaryOr(expressions) => {
                expressions.iter().any(|expression| expression.matches(input))
            }
            NumberMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(input))
            }
            NumberMatchExpression::BinaryNot { expression } => {
                !expression.matches(input)
            }
        }
    }
}

impl<T> Display for NumberMatchExpression<T>
where
    T: Display,
{
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            NumberMatchExpression::Value(value) => {
                write!(f, "{}", value)
            }
            NumberMatchExpression::LessThan { value } => {
                write!(f, "less_than({})", value)
            }
            NumberMatchExpression::LessEquals { value } => {
                write!(f, "less_equals({})", value)
            }
            NumberMatchExpression::GreaterThan { value } => {
                write!(f, "greater_than({})", value)
            }
            NumberMatchExpression::GreaterEquals { value } => {
                write!(f, "greater_equals({})", value)
            }
            NumberMatchExpression::Between { value: (value_low, value_high) } => {
                write!(f, "between({}, {})", value_low, value_high)
            }
            NumberMatchExpression::BinaryOr(expressions) => {
                let param_list =
                    expressions.iter().map(NumberMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "[{}]", param_list)
            }
            NumberMatchExpression::BinaryAnd { expressions } => {
                let param_list =
                    expressions.iter().map(NumberMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "and({})", param_list)
            }
            NumberMatchExpression::BinaryNot { expression } => {
                write!(f, "not({})", expression)
            }
        }
    }
}