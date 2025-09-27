//! All program based on logical propositions.

use core::fmt;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::proposition::PropositionCNF;

/// Represents an atomic proposition, which can be a variable or its negation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Atom {
    /// A positive variable.
    Var(String),

    /// A negative variable.
    NotVar(String),

    /// Represents a boolean value in the expression.
    Value(bool),
}

/// A clause is a disjunction of atoms.
pub struct Clause(pub HashSet<Atom>);

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OptimizedAtom {
    /// A positive variable.
    Var(String),

    /// A negative variable.
    NotVar(String),
}

/// An optimized clause is a disjunction of optimized atoms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizedClause(pub HashSet<OptimizedAtom>);

impl Hash for OptimizedClause {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // To ensure that the hash is independent of the order of atoms in the
        // clause, we can sum the hashes of individual atoms.
        let mut atom_hashes: Vec<u64> = self
            .0
            .iter()
            .map(|atom| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                atom.hash(&mut hasher);
                hasher.finish()
            })
            .collect();
        atom_hashes.sort_unstable();
        for h in atom_hashes {
            h.hash(state);
        }
    }
}

/// Optimizes a list of clauses by applying simplification rules.
pub fn optimized_clauses_from_clauses(clauses: Vec<Clause>) -> HashSet<OptimizedClause> {
    // Rules 1 : If there is a T in the clause, the clause is always true ->
    // Remove the entire clause.
    // Rule 2 : If there is a F in the clause, remove it from the clause.
    // Rule 3 : If there is a variable and its negation in the clause, the
    // clause is always true -> Remove the entire clause.

    // Aply rule 1 and 2
    let mut filtered_clauses: Vec<OptimizedClause> = Vec::new();
    'outer: for clause in clauses {
        let mut new_clause = OptimizedClause(HashSet::new());
        for atom in &clause.0 {
            let new_atom = match atom {
                Atom::Value(true) => continue 'outer,
                Atom::Value(false) => continue,
                Atom::Var(name) => OptimizedAtom::Var(name.clone()),
                Atom::NotVar(name) => OptimizedAtom::NotVar(name.clone()),
            };
            new_clause.0.insert(new_atom);
        }
        if !new_clause.0.is_empty() {
            filtered_clauses.push(new_clause);
        }
    }

    // Aply rule 3
    let mut optimized_clauses: Vec<OptimizedClause> = Vec::new();
    'outer: for clause in filtered_clauses {
        for atom in &clause.0 {
            let negated_atom = match atom {
                OptimizedAtom::Var(name) => OptimizedAtom::NotVar(name.clone()),
                OptimizedAtom::NotVar(name) => OptimizedAtom::Var(name.clone()),
            };
            if clause.0.contains(&negated_atom) {
                continue 'outer;
            }
        }
        optimized_clauses.push(clause);
    }

    optimized_clauses.into_iter().collect()
}

impl Display for OptimizedClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atoms: Vec<String> = self
            .0
            .iter()
            .map(|atom| match atom {
                OptimizedAtom::Var(name) => name.clone(),
                OptimizedAtom::NotVar(name) => format!("¬{}", name),
            })
            .collect();
        write!(f, "{}", atoms.join(" ∨ "))
    }
}
