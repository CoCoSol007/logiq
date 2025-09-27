//! All program based on logical propositions.

use core::fmt;
use std::fmt::Display;

use crate::clause::Atom;

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

/// Represents a logical proposition in Conjunctive Normal Form (CNF).
/// To be more easy to manipulate as clauses it do not support AND operation
/// directly. Instead a CNF is represented as a vector of clauses, where each
/// clause is a disjunction (OR) of literals (variables or their negations).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropositionCNF {
    /// Represents a logical OR operation between two expressions.
    Or(Box<PropositionCNF>, Box<PropositionCNF>),

    /// Represents a boolean value.
    Value(bool),

    /// Represents a variable in the expression.
    Variable(String),

    /// Represents a negated variable in the expression.
    NotVariable(String),
}

impl PropositionCNF {
    /// Converts a PropositionNNF into a vector of PropositionCNF clauses.
    pub fn from_nnf(value: PropositionNNF) -> Vec<Self> {
        match value {
            PropositionNNF::And(a, b) => {
                let mut clauses = PropositionCNF::from_nnf(*a);
                clauses.extend(PropositionCNF::from_nnf(*b));
                clauses
            }
            PropositionNNF::Or(a, b) => {
                let left_clauses = PropositionCNF::from_nnf(*a);
                let right_clauses = PropositionCNF::from_nnf(*b);
                let mut result = Vec::new();
                for left in &left_clauses {
                    for right in &right_clauses {
                        result.push(PropositionCNF::Or(
                            Box::new(left.clone()),
                            Box::new(right.clone()),
                        ));
                    }
                }
                result
            }
            PropositionNNF::Value(v) => vec![PropositionCNF::Value(v)],
            PropositionNNF::Variable(v) => vec![PropositionCNF::Variable(v)],
            PropositionNNF::Not(v) => vec![PropositionCNF::NotVariable(v)],
        }
    }

    /// Extracts the atoms from a CNF expression.
    pub fn get_atoms(self) -> Vec<Atom> {
        match self {
            PropositionCNF::Or(left, right) => {
                let mut left = left.get_atoms();
                let right = right.get_atoms();

                left.extend(right);

                left
            }
            PropositionCNF::Value(v) => vec![Atom::Value(v)],
            PropositionCNF::Variable(s) => vec![Atom::Var(s)],
            PropositionCNF::NotVariable(s) => vec![Atom::NotVar(s)],
        }
    }
}

impl Display for PropositionCNF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_rec(expr: &PropositionCNF) -> String {
            match expr {
                PropositionCNF::Variable(name) => name.clone(),
                PropositionCNF::Value(val) => {
                    if *val {
                        "T".to_string()
                    } else {
                        "F".to_string()
                    }
                }
                PropositionCNF::Or(lhs, rhs) => {
                    format!("({} ∨ {})", fmt_rec(lhs), fmt_rec(rhs))
                }
                PropositionCNF::NotVariable(v) => format!("¬{v}"),
            }
        }

        write!(f, "{}", fmt_rec(self))
    }
}
