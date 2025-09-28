//! Solver module for evaluating logical propositions.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::clause::{
    self, SimplificatedAtom, SimplificatedClause, simplificated_clauses_from_clauses,
};
use crate::proposition::{Proposition, PropositionCNF, PropositionNNF};

/// Represents the solution to a logical proposition evaluation.
pub struct Solution {
    /// A mapping of variable names to their assigned boolean values if the
    /// proposition is satisfiable. If the proposition is unsatisfiable, this
    /// will be None.
    pub assignments: HashSet<Posibility>,
}

impl Solution {
    /// Checks if the proposition is satisfiable based on the assignments.
    pub fn is_satisfiable(&self) -> bool {
        !self.assignments.is_empty()
    }
}

impl From<Vec<Proposition>> for Solution {
    fn from(propositions: Vec<Proposition>) -> Self {
        let mut clauses = Vec::new();

        for proposition in propositions {
            let nnf = PropositionNNF::from(proposition);
            let cnf = PropositionCNF::from_nnf(nnf);
            let mut new_clauses = clause::Clause::from_cnf(cnf);
            clauses.extend(new_clauses.drain(..));
        }

        let simplificated_clauses = simplificated_clauses_from_clauses(clauses);

        let mut assignments = std::collections::HashSet::new();
        backtrack(simplificated_clauses, &mut assignments, HashSet::new());
        Self { assignments }
    }
}

/// Represents a possible assignment of boolean values to variables.
#[derive(Debug, PartialEq, Eq)]
pub struct Posibility(pub HashMap<String, bool>);

impl Hash for Posibility {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut items: Vec<(&String, &bool)> = self.0.iter().collect();
        items.sort_by(|a, b| a.0.cmp(b.0));
        for (key, value) in items {
            key.hash(state);
            value.hash(state);
        }
    }
}

fn backtrack(
    data: HashSet<SimplificatedClause>,
    solutions: &mut HashSet<Posibility>,
    path: HashSet<SimplificatedAtom>,
) {
    if data.is_empty() {
        let mut posibility = Posibility(HashMap::new());
        for atom in path {
            let (k, v) = match atom {
                SimplificatedAtom::Var(v) => (v, true),
                SimplificatedAtom::NotVar(v) => (v, false),
            };
            posibility.0.insert(k, v);
        }
        solutions.insert(posibility);
        return;
    }

    let mut choosen_element = None;
    for a in data.clone() {
        if a.0.len() == 1 {
            choosen_element = a.0.into_iter().next();
        }
    }

    if choosen_element == None {
        choosen_element = data
            .clone()
            .into_iter()
            .next()
            .unwrap()
            .0
            .into_iter()
            .next()
    }

    let choosen_element = choosen_element.unwrap();
    let invered_choosen_element = choosen_element.negate();

    if is_valide(choosen_element.clone(), data.clone()) {
        let mut new_data = HashSet::new();
        for clause in data.clone() {
            if !clause.0.contains(&choosen_element) {
                let mut new_clause = clause.0.clone();
                new_clause.remove(&invered_choosen_element);
                if !new_clause.is_empty() {
                    new_data.insert(SimplificatedClause(new_clause));
                }
            }
        }
        let mut new_path = path.clone();
        new_path.insert(choosen_element.clone());
        backtrack(new_data, solutions, new_path);
    }

    if is_valide(invered_choosen_element.clone(), data.clone()) {
        let mut new_data = HashSet::new();
        for clause in data {
            if !clause.0.contains(&invered_choosen_element) {
                let mut new_clause = clause.0.clone();
                new_clause.remove(&choosen_element);
                if !new_clause.is_empty() {
                    new_data.insert(SimplificatedClause(new_clause));
                }
            }
        }
        let mut new_path = path;
        new_path.insert(invered_choosen_element);
        backtrack(new_data, solutions, new_path);
    }
}

/// Checks if remove a atom is valid in the current context of clauses.
pub fn is_valide(atom: SimplificatedAtom, context: HashSet<SimplificatedClause>) -> bool {
    for clause in context {
        if clause.0.contains(&atom.negate()) && clause.0.len() == 1 {
            return false;
        }
    }
    true
}
