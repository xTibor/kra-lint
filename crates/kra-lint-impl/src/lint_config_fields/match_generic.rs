use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum GenericMatchExpression<T>
where
    T: Display,
{
    Value(T),
    BinaryOr(Vec<GenericMatchExpression<T>>),
    BinaryAnd {
        #[serde(rename = "and")]
        expressions: Vec<GenericMatchExpression<T>>,
    },
    BinaryNot {
        #[serde(rename = "not")]
        expression: Box<GenericMatchExpression<T>>,
    },
}

impl<T> GenericMatchExpression<T>
where
    T: PartialEq<T> + Display,
{
    #[rustfmt::skip]
    pub(crate) fn matches(&self, input: &T) -> bool {
        match self {
            GenericMatchExpression::Value(value) => {
                input == value
            }
            GenericMatchExpression::BinaryOr(expressions) => {
                expressions.iter().any(|expression| expression.matches(input))
            }
            GenericMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(input))
            }
            GenericMatchExpression::BinaryNot { expression } => {
                !expression.matches(input)
            }
        }
    }
}

impl<T> Display for GenericMatchExpression<T>
where
    T: Display,
{
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            GenericMatchExpression::Value(value) => {
                write!(f, "{}", value)
            }
            GenericMatchExpression::BinaryOr(expressions) => {
                let param_list =
                    expressions.iter().map(GenericMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "[{}]", param_list)
            }
            GenericMatchExpression::BinaryAnd { expressions } => {
                let param_list =
                    expressions.iter().map(GenericMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "and({})", param_list)
            }
            GenericMatchExpression::BinaryNot { expression } => {
                write!(f, "not({})", expression)
            }
        }
    }
}
