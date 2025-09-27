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

/// Represents an simplificated atomic proposition, which can be a variable or
/// its negation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimplificatedAtom {
    /// A positive variable.
    Var(String),

    /// A negative variable.
    NotVar(String),
}

/// An simplificated clause is a disjunction of simplificated atoms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimplificatedClause(pub HashSet<SimplificatedAtom>);

impl Hash for SimplificatedClause {
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
pub fn simplificated_clauses_from_clauses(clauses: Vec<Clause>) -> HashSet<SimplificatedClause> {
    // Rules 1 : If there is a T in the clause, the clause is always true ->
    // Remove the entire clause.
    // Rule 2 : If there is a F in the clause, remove it from the clause.
    // Rule 3 : If there is a variable and its negation in the clause, the
    // clause is always true -> Remove the entire clause.

    // Aply rule 1 and 2
    let mut filtered_clauses: Vec<SimplificatedClause> = Vec::new();
    'outer: for clause in clauses {
        let mut new_clause = SimplificatedClause(HashSet::new());
        for atom in &clause.0 {
            let new_atom = match atom {
                Atom::Value(true) => continue 'outer,
                Atom::Value(false) => continue,
                Atom::Var(name) => SimplificatedAtom::Var(name.clone()),
                Atom::NotVar(name) => SimplificatedAtom::NotVar(name.clone()),
            };
            new_clause.0.insert(new_atom);
        }
        if !new_clause.0.is_empty() {
            filtered_clauses.push(new_clause);
        }
    }

    // Aply rule 3
    let mut simplificated_clauses: Vec<SimplificatedClause> = Vec::new();
    'outer: for clause in filtered_clauses {
        for atom in &clause.0 {
            let negated_atom = match atom {
                SimplificatedAtom::Var(name) => SimplificatedAtom::NotVar(name.clone()),
                SimplificatedAtom::NotVar(name) => SimplificatedAtom::Var(name.clone()),
            };
            if clause.0.contains(&negated_atom) {
                continue 'outer;
            }
        }
        simplificated_clauses.push(clause);
    }

    simplificated_clauses.into_iter().collect()
}

impl Display for SimplificatedClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atoms: Vec<String> = self
            .0
            .iter()
            .map(|atom| match atom {
                SimplificatedAtom::Var(name) => name.clone(),
                SimplificatedAtom::NotVar(name) => format!("¬{}", name),
            })
            .collect();
        write!(f, "{}", atoms.join(" ∨ "))
    }
}
