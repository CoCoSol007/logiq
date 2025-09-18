//! TODO

use chumsky::{
    IterParser, Parser,
    prelude::{just, recursive},
    text,
};

use crate::Proposition;

/// Parses a logical expression from a string input.
pub fn parser<'src>() -> impl Parser<'src, &'src str, Proposition> {
    recursive(|expr| {
        // Variables: a-z, A-Z, _
        let variable = text::ident().map(|name: &str| Proposition::Variable(name.to_string()));

        // Constants: true / false
        let boolean = text::keyword("T")
            .to(Proposition::Value(true))
            .or(text::keyword("F").to(Proposition::Value(false)));

        let atom = boolean
            .or(variable)
            .or(expr.clone().delimited_by(just('('), just(')')))
            .padded();

        let not_expr = recursive(|not_expr| {
            just("not")
                .padded()
                .then(not_expr.clone())
                .map(|(_, e)| Proposition::Not(Box::new(e)))
                .or(atom.clone())
        });

        let and_expr = not_expr
            .clone()
            .then(
                just("and")
                    .padded()
                    .then(not_expr.clone())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest)| {
                rest.into_iter().fold(first, |acc, e| {
                    Proposition::And(Box::new(acc), Box::new(e.1))
                })
            });

        let or_expr = and_expr
            .clone()
            .then(
                just("or")
                    .padded()
                    .then(and_expr.clone())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest)| {
                rest.into_iter().fold(first, |acc, e| {
                    Proposition::Or(Box::new(acc), Box::new(e.1))
                })
            });

        let eq_expr = or_expr
            .clone()
            .then(
                just("<->")
                    .padded()
                    .then(or_expr.clone())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest)| {
                rest.into_iter().fold(first, |acc, e| {
                    Proposition::Or(
                        Box::new(Proposition::And(
                            Box::new(acc.clone()),
                            Box::new(e.1.clone()),
                        )),
                        Box::new(Proposition::And(
                            Box::new(Proposition::Not(Box::new(acc))),
                            Box::new(Proposition::Not(Box::new(e.1))),
                        )),
                    )
                })
            });

        let impl_expr = eq_expr
            .clone()
            .then(
                just("->")
                    .padded()
                    .then(eq_expr.clone())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first, rest)| {
                rest.into_iter().fold(first, |acc, e| {
                    Proposition::Or(Box::new(Proposition::Not(Box::new(acc))), Box::new(e.1))
                })
            });

        impl_expr
    })
}
