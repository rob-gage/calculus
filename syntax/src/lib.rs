// Copyright Rob Gage 2025

use num_bigint::BigInt;
use engine::Syntax;
use pups::*;
use std::str::FromStr;

/// Attempts to parse an expression from a `&str` containing expression syntax
pub fn parse_expression(syntax: &str) -> Result<Syntax, ()> {
    let text: Text = Text::from_string(syntax.trim());
    expression(&text)
}


/// Parses an `Syntax` from syntax
fn expression(input: &Text) -> Result<Syntax, ()> {
    choice((
        tertiary,
        token("-").then(whitespace().or_not()).ignore_then(tertiary)
            .map(|negative| Syntax::Multiplication(vec![
                negative, Syntax::Integer (BigInt::from(-1))
            ])),
    )).then(
        repeated(whitespace().or_not().ignore_then(choice((
            token("+").emit(true),
            token("-").emit(false),
        )).then_ignore(whitespace().or_not()).then(tertiary)))
            .map(|vector| vector.into_iter().map(|(positive, term)|
                if positive { term } else { Syntax::Multiplication(vec![
                Syntax::Integer (BigInt::from(-1)), term
            ]) }).collect::<Vec<Syntax>>())
    ).map(|(first, rest)| {
            let mut terms = vec![first];
            terms.extend(rest);
            Syntax::Addition (terms)
        })
        .parse(input)
}


/// Parses a tertiary syntax element (factors, dividends, divisors)
fn tertiary(input: &Text) -> Result<Syntax, ()> {
    choice((
        // `Division`
        secondary.then_ignore(
            delimited(
                whitespace().or_not(),
                token("/"),
                whitespace().or_not()
            )
        ).then(secondary)
            .map(|(dividend, divisor)| Syntax::Division (Box::new((dividend, divisor)))),
        // `Multiplication`
        separated_at_least_once(
            secondary,
            delimited(
                whitespace().or_not(),
                token("*"),
                whitespace().or_not(),
            ),
        )
            .map(|factors| Syntax::Multiplication (factors))
    )).parse(input)
}


/// Parses a secondary syntax element (bases, exponents)
fn secondary(input: &Text) -> Result<Syntax, ()> {
    choice((
        // `Power`
        primary.then_ignore(
            delimited(
                whitespace().or_not(),
                token("^"),
                whitespace().or_not()
            )
        ).then(primary)
            .map(|(base, exponent)| Syntax::Power (Box::new((base, exponent)))),
        primary
    )).parse(input)
}


/// Parses a primary syntax element (named functions, variables, integers, parentheses)
fn primary(input: &Text) -> Result<Syntax, ()> {
    choice((
        // `Exponential`
        delimited(
            token("exp(").then(whitespace().or_not()),
            expression,
            whitespace().or_not().then(token(")")),
        )
            .map(|term| Syntax::Exponential (Box::new(term))),
        // `Logarithm`
        delimited(
            token("log(").then(whitespace().or_not()),
            expression,
            whitespace().or_not().then(token(")")),
        )
            .map(|term| Syntax::Logarithm (Box::new(term))),
        // `Integer`
        number().map(|number| Syntax::Integer (BigInt::from_str(number).unwrap())),
        // `Variable`
        unicode_identifier()
            .map(|identifier: &str| Syntax::Variable (identifier.to_string())),
        parentheses,
    )).parse(input)
}


/// Parses an expression enclosed by parentheses
fn parentheses(input: &Text) -> Result<Syntax, ()> {
    delimited(
        token("(").then(whitespace().or_not()),
        expression,
        whitespace().or_not().then(token(")")),
    )
        .parse(input)
}