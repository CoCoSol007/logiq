//! Logik: A library for parsing and manipulating logical propositions

use crate::clause::optimized_clauses_from_clauses;
use crate::proposition::{Proposition, PropositionCNF, PropositionNNF};

pub mod clause;
pub mod cli;
pub mod lexer;
pub mod parser;
pub mod proposition;

/// Represents the solution to a logical proposition evaluation.
pub struct Solution {
    /// A mapping of variable names to their assigned boolean values if the
    /// proposition is satisfiable. If the proposition is unsatisfiable, this
    /// will be None.
    pub assignments: Option<std::collections::HashMap<String, bool>>,
}

impl Solution {
    /// Checks if the proposition is satisfiable based on the assignments.
    pub fn is_satisfiable(&self) -> bool {
        self.assignments.is_some()
    }

    /// Checks if the proposition is unsatisfiable based on the assignments.
    pub fn is_unsatisfiable(&self) -> bool {
        self.assignments.is_none()
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

        let optimized_clauses = optimized_clauses_from_clauses(clauses);
        for element in &optimized_clauses {
            println!("{}", element);
        }

        todo!()
    }
}
