//! CLI for Logik

use clap::Parser;
use std::path::PathBuf;

/// Logik: a parser and evaluator for logical expressions.
#[derive(Parser)]
#[command(name = "logik", about = "A DSL for first-order logic.")]
pub enum Cli {
    /// Evaluate a logical expression from a file.
    RunFile {
        /// Path to the file containing the logical expression.
        path: PathBuf,
    },

    /// Evaluate a logical expression provided as plain text.
    Run {
        /// Logical expression to parse and evaluate.
        expr: String,
    },
}
