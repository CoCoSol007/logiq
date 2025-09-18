//! ToDo

use core::fmt;

pub mod cli;
pub mod parser;

/// Represents a logical expression.
///
/// Examples
///
/// ``` rust
/// use logik::Expression;
///
/// let expr = Expression::And(
///     Box::new(Expression::Variable("x".to_string())),
///     Box::new(Expression::Value(true)),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropositionGeneralForm {
    /// Represents a negation of an expression.
    Not(Box<PropositionGeneralForm>),

    /// Represents a logical AND operation between two expressions.
    And(Box<PropositionGeneralForm>, Box<PropositionGeneralForm>),

    /// Represents a logical OR operation between two expressions.
    Or(Box<PropositionGeneralForm>, Box<PropositionGeneralForm>),

    /// Represents a boolean value.
    Value(bool),

    /// Represents a variable in the expression.
    Variable(String),
}

impl fmt::Display for PropositionGeneralForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_rec(expr: &PropositionGeneralForm) -> String {
            match expr {
                PropositionGeneralForm::Variable(name) => name.clone(),
                PropositionGeneralForm::Value(val) => {
                    if *val {
                        "T".to_string()
                    } else {
                        "F".to_string()
                    }
                }
                PropositionGeneralForm::Not(inner) => {
                    format!("¬{}", fmt_rec(inner))
                }
                PropositionGeneralForm::And(lhs, rhs) => {
                    format!("({} ∧ {})", fmt_rec(lhs), fmt_rec(rhs))
                }
                PropositionGeneralForm::Or(lhs, rhs) => {
                    format!("({} ∨ {})", fmt_rec(lhs), fmt_rec(rhs))
                }
            }
        }

        write!(f, "{}", fmt_rec(self))
    }
}
