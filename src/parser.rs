//! TODO

use chumsky::error::Rich;
use chumsky::prelude::{just, recursive};
use chumsky::{IterParser, Parser, extra, select};

use crate::Proposition;
use crate::lexer::TokenType;

/// Returns a parser for logical expressions.
pub fn parser<'src>()
-> impl Parser<'src, &'src [TokenType], Vec<Proposition>, extra::Err<Rich<'src, TokenType>>> + Clone
{
    let ident = select!(
        TokenType::Identifier(name) => name.to_owned(),
    )
    .labelled("identifier");

    let expr = recursive(|expr| {
        // 1. **Highest Priority: Atom (Literals, Variables, Parenthesized
        //    Expressions)**
        let boolean = select!(
            TokenType::True => Proposition::Value(true),
            TokenType::False => Proposition::Value(false)
        )
        .labelled("'boolean'");

        // Un 'atom' peut être un booléen, un identifiant ou une expression entre
        // parenthèses.
        let atom = boolean
            .or(ident.clone().map(Proposition::Variable))
            .or(just(TokenType::LParen)
                .ignore_then(expr.clone()) // Un expr complet à l'intérieur des parenthèses
                .then_ignore(just(TokenType::RParen)))
            .labelled("atom");

        // 2. **Priority 5: Not (Unary Operator)**
        // Not lie directement aux atomes.
        let not_expr = recursive(|not_expr| {
            just(TokenType::Not)
                .ignore_then(not_expr.clone().labelled("logical expression"))
                .map(|inner| Proposition::Not(Box::new(inner)))
                .or(atom.clone()) // Ou c'est simplement un atome
                .labelled("'not' expression")
        });

        // Les expressions qui suivent sont de plus en plus faibles et s'appuient sur le
        // niveau de priorité supérieur.

        // 3. **Priority 4: And**
        let and_expr = not_expr
            .clone()
            .labelled("left 'and' expression")
            .then(
                just(TokenType::And)
                    .ignore_then(not_expr.clone().labelled("right 'and' expression"))
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest)| {
                rest.into_iter()
                    .fold(first, |acc, e| Proposition::And(Box::new(acc), Box::new(e)))
            })
            .labelled("'and' expression");

        // 4. **Priority 3: Or**
        let or_expr = and_expr
            .clone()
            .labelled("left 'or' expression")
            .then(
                just(TokenType::Or)
                    .ignore_then(and_expr.clone().labelled("right 'or' expression"))
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest)| {
                rest.into_iter()
                    .fold(first, |acc, e| Proposition::Or(Box::new(acc), Box::new(e)))
            })
            .labelled("'or' expression");

        let impl_expr = or_expr
            .clone()
            .labelled("left implication expression")
            .then(
                just(TokenType::Implication)
                    .ignore_then(or_expr.clone().labelled("right implication expression"))
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest)| {
                rest.into_iter().fold(first, |acc, e| {
                    Proposition::Or(Box::new(Proposition::Not(Box::new(acc))), Box::new(e))
                })
            })
            .labelled("'implication' expression");

        let eq_expr = impl_expr
            .clone()
            .labelled("left equivalence expression")
            .then(
                just(TokenType::Equivalent)
                    .ignore_then(impl_expr.clone().labelled("right equivalence expression"))
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest)| {
                rest.into_iter().fold(first, |acc, e| {
                    Proposition::Or(
                        Box::new(Proposition::And(Box::new(acc.clone()), Box::new(e.clone()))),
                        Box::new(Proposition::And(
                            Box::new(Proposition::Not(Box::new(acc))),
                            Box::new(Proposition::Not(Box::new(e))),
                        )),
                    )
                })
            })
            .labelled("'equivalence' expression");

        eq_expr.labelled("logical expression")
    });

    expr.separated_by(just(TokenType::NewLine).repeated().labelled("new line"))
        .allow_trailing()
        .allow_leading()
        .collect::<Vec<_>>()
}
