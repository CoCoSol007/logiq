//! Logik: A library for parsing and manipulating logical propositions

use core::fmt;
use std::fmt::Display;

pub mod cli;
pub mod lexer;
pub mod parser;

/// Represents a logical proposition in standard form.
///
/// A proposition can be a variable, a boolean constant, or a compound
/// expression built from NOT, AND, and OR operations. This enum supports the
/// full range of propositional logic expressions.
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

/// Represents a logical proposition in Negation Normal Form (NNF).
///
/// In NNF, negations are pushed down to the atomic level, meaning NOT
/// operations can only be applied directly to variables, not to compound
/// expressions. This form is useful for certain logical algorithms and
/// simplifications.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropositionNNF {
    /// Represents a negation of an expression.
    Not(String),

    /// Represents a logical AND operation between two expressions.
    And(Box<PropositionNNF>, Box<PropositionNNF>),

    /// Represents a logical OR operation between two expressions.
    Or(Box<PropositionNNF>, Box<PropositionNNF>),

    /// Represents a boolean value.
    Value(bool),

    /// Represents a variable in the expression.
    Variable(String),
}

impl From<Proposition> for PropositionNNF {
    fn from(value: Proposition) -> Self {
        match value {
            Proposition::And(a, b) => Self::And(
                Box::new(PropositionNNF::from(*a)),
                Box::new(PropositionNNF::from(*b)),
            ),
            Proposition::Or(a, b) => Self::Or(
                Box::new(PropositionNNF::from(*a)),
                Box::new(PropositionNNF::from(*b)),
            ),
            Proposition::Value(v) => Self::Value(v),
            Proposition::Variable(v) => Self::Variable(v),

            Proposition::Not(proposition) => match *proposition {
                Proposition::And(a, b) => Self::Or(
                    Box::new(Proposition::Not(a).into()),
                    Box::new(Proposition::Not(b).into()),
                ),
                Proposition::Or(a, b) => Self::And(
                    Box::new(Proposition::Not(a).into()),
                    Box::new(Proposition::Not(b).into()),
                ),
                Proposition::Not(a) => PropositionNNF::from(*a),
                Proposition::Value(v) => PropositionNNF::Value(!v),
                Proposition::Variable(s) => PropositionNNF::Not(s),
            },
        }
    }
}

impl Display for PropositionNNF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_rec(expr: &PropositionNNF) -> String {
            match expr {
                PropositionNNF::Variable(name) => name.clone(),
                PropositionNNF::Value(val) => {
                    if *val {
                        "T".to_string()
                    } else {
                        "F".to_string()
                    }
                }
                PropositionNNF::Not(inner) => {
                    format!("¬{}", inner)
                }
                PropositionNNF::And(lhs, rhs) => {
                    format!("({} ∧ {})", fmt_rec(lhs), fmt_rec(rhs))
                }
                PropositionNNF::Or(lhs, rhs) => {
                    format!("({} ∨ {})", fmt_rec(lhs), fmt_rec(rhs))
                }
            }
        }

        write!(f, "{}", fmt_rec(self))
    }
}
