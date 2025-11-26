// Copyright Rob Gage 2025

use std::fmt::{
    Display,
    Formatter,
    Result as FormatResult,
    Write,
};
use num_bigint::BigInt;
use num_integer::Integer;
use crate::expression;

/// An algebraic expression
#[derive(Clone)]
pub enum Expression<I: Clone + Eq + PartialEq = usize> {

    /// Addition of terms
    Addition (Vec<Expression<I>>),

    /// Multiplication of terms
    Multiplication (Vec<Expression<I>>),

    /// Division of a term by another
    Division (Box<(Expression<I>, Expression<I>)>),

    /// Exponentiation of a term to another as a power
    Power (Box<(Expression<I>, Expression<I>)>),

    /// Application of the exponential function to a term
    Exponential (Box<Expression<I>>),

    /// Application of the natural logarithm function to a term
    Logarithm (Box<Expression<I>>),

    /// A variable
    Variable (I),

    /// An integer
    Integer (BigInt),

}

impl<I: Clone + Eq + PartialEq> Expression<I> {

    /// Reduce an `Expression`, or returns it unchanged if not reducible
    pub fn reduce(self) -> Self {
        use Expression::*;
        match self {
            Addition (terms) => {
                let terms: Vec<Self> = terms.into_iter()
                    .flat_map(|term| match term {
                        Addition (terms) => terms.into_iter()
                            .map(|term| term.reduce())
                            .collect(),
                        other => vec![other.reduce()]
                    })
                    .collect();
                match terms {
                    // remove unnecessary `Addition` from single term
                    terms if terms.len() == 1 => terms[0].clone(),
                    // convert empty `Addition` to `0`
                    terms if terms.len() == 0 => Integer (BigInt::from(0)),
                    // reduce other `Addition`s
                    terms => {
                        let mut integer_sum: BigInt = BigInt::ZERO;
                        let mut other_terms: Vec<Expression<I>> = Vec::new();
                        for term in terms {
                            match term {
                                Integer (integer) => integer_sum += integer,
                                other => other_terms.push(other.clone()),
                            }
                        }
                        if integer_sum == BigInt::ZERO {
                            if other_terms.is_empty() { return Integer (BigInt::ZERO) }
                        } else { other_terms.push(Integer (integer_sum)) }
                        Addition (other_terms)
                    }
                }
            }
            Multiplication (factors) => {
                let factors: Vec<Self> = factors.into_iter()
                    .flat_map(|term| match term {
                        Multiplication (factors) => factors.into_iter()
                            .map(|term| term.reduce())
                            .collect(),
                        other => vec![other.reduce()]
                    })
                    .collect();
                match factors {
                    // remove unnecessary `Multiplication` from single term
                    factors if factors.len() == 1 => factors[0].clone(),
                    // convert empty `Multiplication` to `0`
                    factors if factors.len() == 0 => Integer (BigInt::from(0)),
                    // reduce other `Multiplication`s
                    terms => {
                        let mut integer_product: BigInt = BigInt::from(1);
                        let mut other_terms: Vec<Expression<I>> = Vec::new();
                        for term in terms {
                            match term {
                                Integer (integer) => integer_product *= integer,
                                other => other_terms.push(other.clone()),
                            }
                        }
                        if integer_product == BigInt::ZERO {
                            return Integer (BigInt::ZERO);
                        } else if integer_product != BigInt::from(1) {
                            other_terms.push(Integer (integer_product));
                        }
                        Multiplication (other_terms)
                    }
                }
            }
            Division (terms) => {
                let dividend: Expression<I> = terms.0.reduce();
                let divisor: Expression<I> = terms.1.reduce();
                match (&dividend, &divisor) {
                    // reduce fractions
                    (Integer (numerator), Integer (denominator)) => {
                        let gcd: BigInt = numerator.gcd(&denominator);
                        let numerator: BigInt = numerator / &gcd;
                        let denominator: BigInt = denominator / &gcd;
                        if denominator == BigInt::from(1) { Integer (numerator) } else {
                            Division (Box::new((Integer (numerator), Integer (denominator))))
                        }
                    }
                    _ => Division (Box::new((dividend, divisor))),
                }
            }
            Power (terms) => {
                let base: Expression<I> = terms.0.reduce();
                let exponent: Expression<I> = terms.1.reduce();
                Power (Box::new((base, exponent)))
            }
            other => other
        }
    }

    /// Differentiates this `Expression` with respect to a variable
    pub fn differentiate(&self, variable: &I) -> Self {
        use Expression::*;
        match self {
            // identity rule
            Variable (identifier) if identifier == variable => Integer (BigInt::from(1)),
            // variable rule
            Variable (_) => Integer (BigInt::from(0)),
            // constant rule
            Integer (_) => Integer (BigInt::from(0)),
            // sum rule
            Addition (terms) => Addition (terms.iter()
                .map(|operand| operand.differentiate(variable))
                .collect()
            ),
            // product rule
            Multiplication (factors) => Addition (factors.iter()
                .enumerate()
                .map(|(factor_index, factor)| {
                    let mut output: Vec<Expression<I>> = Vec::with_capacity(factors.len());
                    output.push(factor.differentiate(variable));
                    for index in 0..factors.len() {
                        if index != factor_index {
                            output.push(factors[index].clone());
                        }
                    }
                    Multiplication (output)
                })
                .collect()
            ),
            // quotient rule
            Division (terms) => Division (Box::new((
                Addition (vec![
                    Multiplication (vec![terms.0.differentiate(variable), terms.1.clone()]),
                    Multiplication (vec![terms.0.clone(), terms.1.differentiate(variable)]),
                ]),
                Multiplication (vec![terms.1.clone(), terms.1.clone()])
            ))),
            // power rules
            Power (terms) => match *terms.clone() {
                // known base shortcut
                (Integer (base), exponent) => Multiplication(vec![
                    Power (Box::new((Integer (base.clone()), exponent.clone()))),
                    Logarithm (Box::new(Integer (base))),
                    exponent.differentiate(variable)
                ]),
                // known exponent shortcut
                (base, Integer (exponent)) => if exponent == BigInt::ZERO {
                    Integer (BigInt::ZERO)
                } else if exponent == BigInt::from(1) {
                    base.differentiate(variable)
                } else { Multiplication (vec![
                    Integer (exponent.clone()),
                    Power (Box::new ((base.clone(), Integer (exponent - 1)))),
                    base.differentiate(variable)
                ])},
                // general power rule
                (base, exponent) => Multiplication (vec![
                    Power (Box::new((base.clone(), exponent.clone()))),
                    Addition (vec![
                        Multiplication (vec![
                            exponent.differentiate(variable),
                            Logarithm (Box::new(base.clone()))
                        ]),
                        Multiplication (vec![
                            exponent,
                            Division(Box::new((base.differentiate(variable), base)))
                        ])
                    ])
                ])
            }
            // exponential rule
            Exponential (term) => Multiplication (vec![
                Exponential (term.clone()),
                term.differentiate(variable)
            ]),
            // logarithm rule
            Logarithm (term) => Division (Box::new((
                term.differentiate(variable),
                *term.clone(),
            ))),
        }
    }

}

impl Display for Expression<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        use Expression::*;
        match self {
            Addition (terms) => {
                for index in 0..terms.len() {
                    if index != 0 { f.write_str(" + ")?; }
                    write!(f, "{}", terms[index])?;
                }
                Ok(())
            }
            Multiplication (terms) => {
                for index in 0..terms.len() {
                    if index != 0 { f.write_str(" \\cdot ")?; }
                    write!(f, "{}", terms[index])?;
                }
                Ok(())
            }
            Division (operands) => write!(
                f,
                "\\displaystyle \\frac{{{}}}{{{}}}",
                operands.0,operands.1),
            Power (operands) => write!(f, "{}^{{{}}}", operands.0, operands.1),
            Exponential (operand) => write!(f, "e^{{{}}}", operand),
            Logarithm (operand) => write!(f, "\\ln({})", operand),
            Variable (name) => f.write_str(name),
            Integer (integer) => f.write_str(&integer.to_string()),
        }
    }
}