// Copyright Rob Gage 2025

use num_bigint::BigInt;
use core::Expression;
use pups::*;
use std::str::FromStr;

// /// Parses an `Expression` from syntax
// pub fn parse_syntax<'a>(input: &'a Text) -> ParseResult<Expression, String> {
//     
// }
// 
// /// Parses an `Expression::Variable` from syntax
// pub fn parse_variable<'a>() -> impl Parser<'a, Expression, String, (), Text> {
//     unicode_identifier()
//         .map(|identifier: &str| Expression::Variable (identifier.to_string()))
//         .map_error(|_| "".to_string())
// }
// 
// /// Parses an `Expression::Integer` from syntax
// pub fn integer_parser<'a>() -> impl Parser<'a, Expression, String, (), Text> {
//     token("-").or_not()
//         .then_ignore(whitespace().or_not())
//         .then(number())
//         .map(|(sign, number)| Expression::Integer (BigInt::from_str(number).unwrap()
//             * BigInt::from(if sign.is_some() { -1 } else { 1 }))
//         )
//         .map_error(|_| "".to_string())
// }