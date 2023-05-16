use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum LintGenericMatchExpression<T>
where
    T: Display,
{
    Value(T),
    BinaryOr(Vec<LintGenericMatchExpression<T>>),
    BinaryAnd {
        #[serde(rename = "and")]
        expressions: Vec<LintGenericMatchExpression<T>>,
    },
    BinaryNot {
        #[serde(rename = "not")]
        expression: Box<LintGenericMatchExpression<T>>,
    },
}

impl<T> LintGenericMatchExpression<T>
where
    T: PartialEq<T> + Display,
{
    #[rustfmt::skip]
    pub(crate) fn matches(&self, input: &T) -> bool {
        match self {
            LintGenericMatchExpression::Value(value) => {
                input == value
            }
            LintGenericMatchExpression::BinaryOr(expressions) => {
                expressions.iter().any(|expression| expression.matches(input))
            }
            LintGenericMatchExpression::BinaryAnd { expressions } => {
                expressions.iter().all(|expression| expression.matches(input))
            }
            LintGenericMatchExpression::BinaryNot { expression } => {
                !expression.matches(input)
            }
        }
    }
}

impl<T> Display for LintGenericMatchExpression<T>
where
    T: Display,
{
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LintGenericMatchExpression::Value(value) => {
                write!(f, "{}", value)
            }
            LintGenericMatchExpression::BinaryOr(expressions) => {
                let param_list =
                    expressions.iter().map(LintGenericMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "[{}]", param_list)
            }
            LintGenericMatchExpression::BinaryAnd { expressions } => {
                let param_list =
                    expressions.iter().map(LintGenericMatchExpression::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "and({})", param_list)
            }
            LintGenericMatchExpression::BinaryNot { expression } => {
                write!(f, "not({})", expression)
            }
        }
    }
}
