// Copyright Rob Gage 2025

use num_bigint::BigInt;
use core::Expression;
use pups::*;
use std::str::FromStr;

/// Attempts to parse an expression from a `&str` containing expression syntax
pub fn parse_expression(syntax: &str) -> Result<Expression, ()> {
    let text: Text = Text::from_string(syntax.trim());
    expression(&text)
}


/// Parses an `Expression` from syntax
fn expression(input: &Text) -> Result<Expression, ()> {
    choice((
        tertiary.then_ignore(
            delimited(
                whitespace().or_not(),
                token("-"),
                whitespace().or_not()
            )
        ).then(tertiary)
            .map(|(left, right)| Expression::Subtraction (Box::new((left, right)))),
        // `Addition`
        separated(
            tertiary,
            delimited(
                whitespace().or_not(),
                token("+"),
                whitespace().or_not(),
            ),
        )
            .map(|terms| Expression::Addition (terms)),
    )).parse(input)
}

/// Parses a tertiary syntax element (factors, dividends, divisors)
fn tertiary(input: &Text) -> Result<Expression, ()> {
    choice((
        // `Division`
        secondary.then_ignore(
            delimited(
                whitespace().or_not(),
                token("/"),
                whitespace().or_not()
            )
        ).then(secondary)
            .map(|(dividend, divisor)| Expression::Division (Box::new((dividend, divisor)))),
        // `Multiplication`
        separated(
            secondary,
            delimited(
                whitespace().or_not(),
                token("*"),
                whitespace().or_not(),
            ),
        )
            .map(|factors| Expression::Multiplication (factors))
    )).parse(input)
}

/// Parses a secondary syntax element (bases, exponents)
fn secondary(input: &Text) -> Result<Expression, ()> {
    choice((
        // `Power`
        primary.then_ignore(
            delimited(
                whitespace().or_not(),
                token("^"),
                whitespace().or_not()
            )
        ).then(primary)
            .map(|(base, exponent)| Expression::Power (Box::new((base, exponent)))),
        primary
    )).parse(input)
}

/// Parses a primary syntax element (named functions, variables, integers, parentheses)
fn primary(input: &Text) -> Result<Expression, ()> {
    choice((
        // `Exponential`
        delimited(
            token("exp(").then(whitespace().or_not()),
            expression,
            whitespace().or_not().then(token(")")),
        )
            .map(|term| Expression::Exponential (Box::new(term))),
        // `Logarithm`
        delimited(
            token("log(").then(whitespace().or_not()),
            expression,
            whitespace().or_not().then(token(")")),
        )
            .map(|term| Expression::Logarithm (Box::new(term))),
        // `Integer`
        token("-").or_not().then(number())
            .map(|(sign, number)| Expression::Integer (BigInt::from_str(number).unwrap()
                * BigInt::from(if sign.is_some() { -1 } else { 1 }))
            ),
        // `Variable`
        unicode_identifier()
            .map(|identifier: &str| Expression::Variable (identifier.to_string())),
        parentheses,
    )).parse(input)
}


/// Parses an expression enclosed by parentheses
fn parentheses(input: &Text) -> Result<Expression, ()> {
    delimited(
        token("(").then(whitespace().or_not()),
        expression,
        whitespace().or_not().then(token(")")),
    )
        .parse(input)
}