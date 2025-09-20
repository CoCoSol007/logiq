//! Main entry point for the logik DSL.

use std::fs;

use chumsky::Parser;
use logik::PropositionNNF;
use logik::cli::Cli;

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

    let mut usable_line = vec![];
    for line in file_content.split("\n") {
        if !line.starts_with("//") {
            usable_line.push(line.split("//").next().unwrap_or_default());
        }
    }

    let text = usable_line.join("\n");

    let parser = logik::parser::parser();
    let ast = parser.parse(&text).unwrap();
    let nnf = PropositionNNF::from(ast);
    println!("{}", nnf);
}
