// Copyright Rob Gage 2025

use num_bigint::BigInt;
use core::Expression;
use pups::*;
use std::str::FromStr;

/// Attempts to parse an expression from a `&str` containing expression syntax
fn parse_expression(syntax: &str) -> Result<Expression, ()> {
    let text: Text = Text::from_string(syntax.trim());
    expression(&text)
}

/// Parses an `Expression` from syntax
fn expression(input: &Text) -> Result<Expression, ()> {
    choice((
        parentheses,
        integer,
        variable,
        addition,
        subtraction,
    )).parse(input)
}


/// Parses an `Expression` enclosed in parentheses
fn parentheses(input: &Text) -> Result<Expression, ()> {
    delimited(
        token("(").then(whitespace().or_not()),
        expression,
        whitespace().or_not().then(token(")")),
    )
        .parse(input)
}


/// Parses an `Expression::Integer` from syntax
fn integer(input: &Text) -> Result<Expression, ()> {
    token("-").or_not()
        .then_ignore(whitespace().or_not())
        .then(number())
        .map(|(sign, number)| Expression::Integer (BigInt::from_str(number).unwrap()
            * BigInt::from(if sign.is_some() { -1 } else { 1 }))
        )
        .parse(input)
}


/// Parses an `Expression::Variable` from syntax
fn variable(input: &Text) -> Result<Expression, ()> {
    unicode_identifier()
        .map(|identifier: &str| Expression::Variable (identifier.to_string()))
        .parse(input)
}


/// Parses an `Expression::Addition` from syntax
fn addition(input: &Text) -> Result<Expression, ()> {
    separated(
        expression,
        delimited(
            whitespace().or_not(),
            token("+"),
            whitespace().or_not(),
        ),
    )
        .map(|terms| Expression::Addition (terms))
        .parse(input)
}


/// Parses an `Expression::Subtraction` from syntax
fn subtraction(input: &Text) -> Result<Expression, ()> {
    expression.then_ignore(
        delimited(
            whitespace().or_not(),
            token("-"),
            whitespace().or_not()
        )
    ).then(expression)
        .map(|(left, right)| Expression::Subtraction (Box::new((left, right))))
        .parse(input)
}


/// Parses an `Expression::Multiplication` from syntax
fn multiplication(input: &Text) -> Result<Expression, ()> {
    separated(
        expression,
        delimited(
            whitespace().or_not(),
            token("*"),
            whitespace().or_not(),
        ),
    )
        .map(|factors| Expression::Multiplication (factors))
        .parse(input)
}


/// Parses an `Expression::Division` from syntax
fn division(input: &Text) -> Result<Expression, ()> {
    expression.then_ignore(
        delimited(
            whitespace().or_not(),
            token("/"),
            whitespace().or_not()
        )
    ).then(expression)
        .map(|(dividend, divisor)| Expression::Division (Box::new((dividend, divisor))))
        .parse(input)
}


/// Parses an `Expression::Power` from syntax
fn power(input: &Text) -> Result<Expression, ()> {
    expression.then_ignore(
        delimited(
            whitespace().or_not(),
            token("^"),
            whitespace().or_not()
        )
    ).then(expression)
        .map(|(base, exponent)| Expression::Power (Box::new((base, exponent))))
        .parse(input)
}


/// Parses an `Expression::Power` from syntax
fn exponential(input: &Text) -> Result<Expression, ()> {
    delimited(
        token("exp(").then(whitespace().or_not()),
        expression,
        whitespace().or_not().then(token(")")),
    )
        .map(|term| Expression::Exponential (Box::new(term)))
        .parse(input)
}


/// Parses an `Expression::Power` from syntax
fn logarithm(input: &Text) -> Result<Expression, ()> {
    delimited(
        token("log(").then(whitespace().or_not()),
        expression,
        whitespace().or_not().then(token(")")),
    )
        .map(|term| Expression::Exponential (Box::new(term)))
        .parse(input)
}