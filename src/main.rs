//! Main entry point for the logiq DSL.

use std::fs;
use std::process::exit;

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::Parser;
use chumsky::error::Rich;
use logos::Logos;

use crate::cli::Cli;
use crate::lexer::TokenType;
use crate::solver::{SolveError, solve};

mod clause;
mod cli;
mod lexer;
mod parser;
mod proposition;
mod solver;

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    let mut path: String = "<input>".into();

    let file_content_result = match cli {
        Cli::RunFile { path: p } => {
            path = p.to_str().unwrap_or("<input>").into();
            fs::read_to_string(p.clone())
        }
        Cli::Run { expr } => Ok(expr),
    };

    if let Err(error) = file_content_result {
        println!("Error: {}", error);
        return;
    }

    let content = file_content_result.unwrap();

    let mut tokens = Vec::new();
    let mut tokens_type = Vec::new();

    for (result_token_type, span) in TokenType::lexer(&content).spanned() {
        let Ok(token_type) = result_token_type else {
            Report::build(ReportKind::Error, (path.clone(), span.clone()))
                .with_message("Lexing Error")
                .with_label(Label::new((path.clone(), span)).with_message("unrecognized token"))
                .finish()
                .print((path.clone(), Source::from(&content)))
                .unwrap();
            exit(1);
        };
        let token = lexer::Token {
            token_type: token_type.clone(),
            span,
        };
        tokens_type.push(token_type.clone());
        tokens.push(token);
    }

    let parser = parser::parser();
    match parser.parse(tokens_type.as_slice()).into_result() {
        Ok(propositions) => {
            Report::build(
                ReportKind::Custom("Info", ariadne::Color::Green),
                (path.clone(), 0..0),
            )
            .with_message("Lexing & Parsing Successful")
            .finish()
            .print((path.clone(), Source::from(&content)))
            .unwrap();

            match solve(propositions) {
                Ok(posibilities) => {
                    println!("\nThe proposition is satisfiable.");
                    println!("Possible assignments:\n");

                    for (i, possibility) in posibilities.iter().enumerate() {
                        println!("-- Possibility #{} --", i + 1);

                        for (var, value) in &possibility.0 {
                            println!("  {} = {}", var, value);
                        }

                        println!();
                    }
                }
                Err(SolveError::Unsatisfiable) => println!("\nThe proposition is unsatisfiable."),
                Err(SolveError::NoVariable) => println!("\nThe proposition has no variables."),
            };
        }
        Err(errors) => handle_error_file(errors, &path, &content, &tokens),
    }
}

/// Handle parser errors by reporting them with ariadne and exiting the
/// program.
fn handle_error_file(
    errors: Vec<Rich<TokenType>>,
    file_path: &str,
    source: &str,
    tokens: &Vec<lexer::Token>,
) {
    for e in errors {
        let span_token_type: std::ops::Range<usize> = e.span().into_iter();
        let span: std::ops::Range<usize> = match tokens.get(span_token_type.start) {
            Some(t_start) => match tokens.get(span_token_type.end - 1) {
                Some(t_end) => t_start.span.start..t_end.span.end,
                None => t_start.span.start..t_start.span.end,
            },
            // We suppose that if there is no the element in the tokens vector, the error
            // is about a missing token at the end of the vector.
            None => tokens
                .last()
                .map(|t| t.span.end..(t.span.end))
                .unwrap_or(0..0),
        };
        Report::build(ReportKind::Error, (file_path, span.clone()))
            .with_message("Parser Error")
            .with_label(Label::new((file_path, span)).with_message(format!(
                "expected {}",
                e.expected().map(|f| f.to_string()).collect::<Vec<_>>().join(", ")
            )))
            .finish()
            .print((file_path, Source::from(source)))
            .unwrap();
    }
    exit(1);
}
