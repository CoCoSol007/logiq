//! All program based on logical propositions.

use core::fmt;
use std::fmt::Display;

use crate::proposition::PropositionCNF;

/// Represents an atomic proposition, which can be a variable or its negation.
pub enum Atom {
    /// A positive variable.
    Var(String),

    /// A negative variable.
    NotVar(String),

    /// Represents a boolean value in the expression.
    Value(bool),
}

/// A clause is a disjunction of atoms.
pub struct Clause(pub Vec<Atom>);

impl Clause {
    /// Converts a CNF expression into a vector of clauses.
    pub fn from_cnf(cnf: Vec<PropositionCNF>) -> Vec<Self> {
        cnf.into_iter()
            .map(|prop| Clause(prop.get_atoms()))
            .collect()
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atoms: Vec<String> = self
            .0
            .iter()
            .map(|atom| match atom {
                Atom::Var(name) => name.clone(),
                Atom::NotVar(name) => format!("¬{}", name),
                Atom::Value(v) => {
                    if *v {
                        "T".to_string()
                    } else {
                        "F".to_string()
                    }
                }
            })
            .collect();
        write!(f, "{}", atoms.join(" ∨ "))
    }
}

/// Represents an optimized atomic proposition, which can be a variable or its
/// negation.
pub enum OptimizedAtom {
    /// A positive variable.
    Var(String),

    /// A negative variable.
    NotVar(String),
}

/// An optimized clause is a disjunction of optimized atoms.
pub struct OptimizedClause(pub Vec<OptimizedAtom>);

impl From<Clause> for OptimizedClause {
    fn from(clause: Clause) -> Self {
        todo!()
    }
}
