//! Main entry point for the logik DSL.

use logik::{PropositionNNF, cli::Cli};
use std::fs;

use chumsky::Parser;

fn main() {
    let cli = <Cli as clap::Parser>::parse();

    let file_content_result = match cli {
        Cli::RunFile { path } => fs::read_to_string(path),
        Cli::Run { expr } => Ok(expr),
    };

    if let Err(error) = file_content_result {
        println!("Error: {}", error);
        return;
    }

    let file_content = file_content_result.unwrap();
    let parser = logik::parser::parser();
    let ast = parser.parse(&file_content).unwrap();
    let _nnf = PropositionNNF::from(ast);
}
