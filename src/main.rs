//! Main entry point for the logik DSL.

use std::fs;
use std::process::exit;

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::Parser;
use chumsky::error::Rich;
use logik::cli::Cli;
use logik::lexer::TokenType;
use logos::Logos;

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    let mut path = None;

    let file_content_result = match cli {
        Cli::RunFile { path: p } => {
            path = Some(p.clone());
            fs::read_to_string(p)
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
            let file_path = match &path {
                Some(p) => p.to_string_lossy(),
                None => "<input>".into(),
            };
            Report::build(ReportKind::Error, (file_path.clone(), span.clone()))
                .with_message("Lexing Error")
                .with_label(
                    Label::new((file_path.clone(), span)).with_message("unrecognized token"),
                )
                .finish()
                .print((file_path, Source::from(&content)))
                .unwrap();
            exit(1);
        };
        let token = logik::lexer::Token {
            token_type: token_type.clone(),
            span,
        };
        tokens_type.push(token_type.clone());
        println!("{:?} at {:?}", token.token_type, token.span);
        tokens.push(token);
    }

    let parser = logik::parser::parser();
    match parser.parse(tokens_type.as_slice()).into_result() {
        Ok(ast) => {
            for proposition in ast {
                println!("AST: {}", proposition);
            }
        }
        Err(errors) => match &path {
            Some(p) => handle_error_file(errors, &p.to_string_lossy(), &content, &tokens),
            None => handle_error_file(errors, "<input>", &content, &tokens),
        },
    }
}

/// Handle parser errors by reporting them with ariadne and exiting the
/// program.
fn handle_error_file(
    errors: Vec<Rich<TokenType>>,
    file_path: &str,
    source: &str,
    tokens: &Vec<logik::lexer::Token>,
) {
    for e in errors {
        let span_token_type: std::ops::Range<usize> = e.span().into_iter();
        let span: std::ops::Range<usize> = match tokens.get(span_token_type.start) {
            Some(t_start) => match tokens.get(span_token_type.end - 1) {
                Some(t_end) => t_start.span.start..t_end.span.end,
                None => t_start.span.start..t_start.span.end,
            },
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
