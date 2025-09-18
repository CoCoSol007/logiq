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
pub enum Proposition {
    /// Represents a negation of an expression.
    Not(Box<Proposition>),

    /// Represents a logical AND operation between two expressions.
    And(Box<Proposition>, Box<Proposition>),

    /// Represents a logical OR operation between two expressions.
    Or(Box<Proposition>, Box<Proposition>),

    /// Represents a boolean value.
    Value(bool),

    /// Represents a variable in the expression.
    Variable(String),
}

impl fmt::Display for Proposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_rec(expr: &Proposition) -> String {
            match expr {
                Proposition::Variable(name) => name.clone(),
                Proposition::Value(val) => {
                    if *val {
                        "T".to_string()
                    } else {
                        "F".to_string()
                    }
                }
                Proposition::Not(inner) => {
                    format!("¬{}", fmt_rec(inner))
                }
                Proposition::And(lhs, rhs) => {
                    format!("({} ∧ {})", fmt_rec(lhs), fmt_rec(rhs))
                }
                Proposition::Or(lhs, rhs) => {
                    format!("({} ∨ {})", fmt_rec(lhs), fmt_rec(rhs))
                }
            }
        }

        write!(f, "{}", fmt_rec(self))
    }
}
