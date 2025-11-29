// Copyright Rob Gage 2025

use num::{
    bigint::BigInt,
    integer::Integer,
    traits::ToPrimitive,
};
use std::{
    f64::consts::E,
    fmt::{
        Display,
        Formatter,
        Result as FormatResult,
        Write,
    }
};

/// An algebraic expression
#[derive(Clone)]
pub enum Expression<I: Clone + Eq + PartialEq = usize> {

    /// Addition of terms
    Sum (Vec<Expression<I>>),

    /// Multiplication of terms
    Product (Vec<Expression<I>>),

    /// Division of a term by another
    Quotient (Box<(Expression<I>, Expression<I>)>),

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

    /// Evaluates an `Expression` with a list of input values for a given variable
    ///
    /// (This method requires that no other unsubstituted variables remain in the `Expression`)
    pub fn evaluate(&self, variable: &I, values: &[f64]) -> Result<Vec<f64>, ()> {
        match self {
            Expression::Sum(terms) => {
                let mut output: Vec<f64> = vec![0.0; values.len()];
                for term in terms {
                    let term_values: Vec<f64> = term.evaluate(variable, values)?;
                    for (a, b) in output.iter_mut().zip(term_values) {
                        *a += b;
                    }
                }
                Ok (output)
            }
            Expression::Product(factors) => {
                let mut output: Vec<f64> = vec![1.0; values.len()];
                for factor in factors {
                    let term_values: Vec<f64> = factor.evaluate(variable, values)?;
                    for (a, b) in output.iter_mut().zip(term_values) {
                        *a *= b;
                    }
                }
                Ok (output)
            }
            Expression::Quotient(operands) => Ok (
                operands.0.evaluate(variable, values)?.into_iter()
                    .zip(operands.1.evaluate(variable, values)?.into_iter())
                    .map(|(a, b)| a / b)
                    .collect()
            ),
            Expression::Power (operands) => Ok (
                operands.0.evaluate(variable, values)?.into_iter()
                    .zip(operands.1.evaluate(variable, values)?.into_iter())
                    .map(|(a, b)| a.powf(b))
                    .collect()
            ),
            Expression::Exponential (operand) => Ok (
                operand.evaluate(variable, values)?.into_iter()
                    .map(|value| E.powf(value))
                    .collect()
            ),
            Expression::Logarithm (operand) => Ok (
                operand.evaluate(variable, values)?.into_iter()
                    .map(|value| value.ln())
                    .collect()
            ),
            Expression::Variable (identifier) if identifier == variable => Ok (values.to_vec()),
            Expression::Integer (integer) => {
                let float: f64 = integer.to_f64().unwrap_or(f64::NAN);
                Ok (vec![float; values.len()])
            }
            _ => Err (())
        }
    }

    /// Reduce an `Expression`, or returns it unchanged if not reducible
    pub fn reduce(self) -> Self {
        use Expression::*;
        match self {
            Sum(terms) => {
                let terms: Vec<Self> = terms.into_iter()
                    .flat_map(|term| match term {
                        Sum(terms) => terms.into_iter()
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
                        Sum(other_terms)
                    }
                }
            }
            Product(factors) => {
                let factors: Vec<Self> = factors.into_iter()
                    .flat_map(|term| match term {
                        Product(factors) => factors.into_iter()
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
                        Product(other_terms)
                    }
                }
            }
            Quotient(terms) => {
                let dividend: Expression<I> = terms.0.reduce();
                let divisor: Expression<I> = terms.1.reduce();
                match (&dividend, &divisor) {
                    // reduce fractions
                    (Integer (numerator), Integer (denominator)) => {
                        let gcd: BigInt = numerator.gcd(&denominator);
                        let numerator: BigInt = numerator / &gcd;
                        let denominator: BigInt = denominator / &gcd;
                        if denominator == BigInt::from(1) { Integer (numerator) } else {
                            Quotient(Box::new((Integer (numerator), Integer (denominator))))
                        }
                    }
                    _ => Quotient(Box::new((dividend, divisor))),
                }
            }
            Power (terms) => {
                let base: Expression<I> = terms.0.reduce();
                let exponent: Expression<I> = terms.1.reduce();
                match (&base, &exponent) {
                    (Integer (base), Integer (exponent))
                    if exponent <= &BigInt::from(u32::MAX)
                    && exponent >= &BigInt::ZERO =>
                        Integer (base.pow(exponent.to_u32_digits().1[0])),
                    _ => Power (Box::new((base, exponent)))
                }
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
            Sum(terms) => Sum(terms.iter()
                .map(|operand| operand.differentiate(variable))
                .collect()
            ),
            // product rule
            Product(factors) => Sum(factors.iter()
                .enumerate()
                .map(|(factor_index, factor)| {
                    let mut output: Vec<Expression<I>> = Vec::with_capacity(factors.len());
                    output.push(factor.differentiate(variable));
                    for index in 0..factors.len() {
                        if index != factor_index {
                            output.push(factors[index].clone());
                        }
                    }
                    Product(output)
                })
                .collect()
            ),
            // quotient rule
            Quotient(terms) => Quotient(Box::new((
                Sum(vec![
                    Product(vec![terms.0.differentiate(variable), terms.1.clone()]),
                    Product(vec![terms.0.clone(), terms.1.differentiate(variable)]),
                ]),
                Product(vec![terms.1.clone(), terms.1.clone()])
            ))),
            // power rules
            Power (terms) => match *terms.clone() {
                // known base shortcut
                (Integer (base), exponent) => Product(vec![
                    Power (Box::new((Integer (base.clone()), exponent.clone()))),
                    Logarithm (Box::new(Integer (base))),
                    exponent.differentiate(variable)
                ]),
                // known exponent shortcut
                (base, Integer (exponent)) => if exponent == BigInt::ZERO {
                    Integer (BigInt::ZERO)
                } else if exponent == BigInt::from(1) {
                    base.differentiate(variable)
                } else { Product(vec![
                    Integer (exponent.clone()),
                    Power (Box::new ((base.clone(), Integer (exponent - 1)))),
                    base.differentiate(variable)
                ])},
                // general power rule
                (base, exponent) => Product(vec![
                    Power (Box::new((base.clone(), exponent.clone()))),
                    Sum(vec![
                        Product(vec![
                            exponent.differentiate(variable),
                            Logarithm (Box::new(base.clone()))
                        ]),
                        Product(vec![
                            exponent,
                            Quotient(Box::new((base.differentiate(variable), base)))
                        ])
                    ])
                ])
            }
            // exponential rule
            Exponential (term) => Product(vec![
                Exponential (term.clone()),
                term.differentiate(variable)
            ]),
            // logarithm rule
            Logarithm (term) => Quotient(Box::new((
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
            Sum(terms) => {
                for index in 0..terms.len() {
                    if index != 0 { f.write_str(" + ")?; }
                    write!(f, "{}", terms[index])?;
                }
                Ok(())
            }
            Product(terms) => {
                for index in 0..terms.len() {
                    if index != 0 { f.write_str(" \\cdot ")?; }
                    write!(f, "{}", terms[index])?;
                }
                Ok(())
            }
            Quotient(operands) => write!(
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