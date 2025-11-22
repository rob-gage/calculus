// Copyright Rob Gage 2025

use num_bigint::BigInt;
use num_integer::Integer;

/// An algebraic expression
#[derive(Clone)]
pub enum Expression {

    /// Addition of terms
    Addition (Vec<Expression>),
    /// Subtraction of a term from another
    Subtraction (Box<(Expression, Expression)>),
    /// Multiplication of terms
    Multiplication (Vec<Expression>),
    /// Division of a term by another
    Division (Box<(Expression, Expression)>),

    /// Exponentiation of a term to another as a power
    Power (Box<(Expression, Expression)>),
    /// Application of the exponential function to a term
    Exponential (Box<Expression>),
    /// Application of the natural logarithm function to a term
    Logarithm (Box<Expression>),

    /// A variable
    Variable (String),
    /// An integer
    Integer (BigInt),

}

impl Expression {

    /// Flattens a potentially nested `Expression`, or returns it unchanged if not nested
    fn flatten(self) -> Self {
        use Expression::*;
        match self {
            Addition (terms) => {
                let mut flattened: Vec<Expression> = Vec::new();
                for term in terms {
                    match term {
                        Addition (addition_terms) => flattened.extend(
                            addition_terms.into_iter().map(|term| term.flatten())
                        ),
                        other => flattened.push(other.flatten()),
                    }
                }
                Addition (flattened)
            },
            Subtraction (terms) => Subtraction(Box::new((terms.0.flatten(), terms.1.flatten()))),
            Multiplication (terms) => {
                let mut flattened: Vec<Expression> = Vec::new();
                for term in terms {
                    match term {
                        Multiplication (multiplication_terms) => flattened.extend(
                            multiplication_terms.into_iter().map(|term| term.flatten())
                        ),
                        other => flattened.push(other.flatten()),
                    }
                }
                Multiplication (flattened)
            }
            Division (terms) => Division(Box::new((
                terms.0.flatten(),
                terms.1.flatten(),
            ))),
            other => other,
        }
    }

    /// Reduce an `Expression`, or returns it unchanged if not reducible
    fn reduce(self) -> Self {
        use Expression::*;
        let flattened: Self = self.flatten();
        match flattened {
            Addition (terms) => {
                let terms: Vec<Self> = terms.into_iter()
                    .map(|term| term.reduce())
                    .collect();
                match terms {
                    // remove unnecessary `Addition` from single term
                    terms if terms.len() == 1 => terms[0].clone(),
                    // convert empty `Addition` to `0`
                    terms if terms.len() == 0 => Integer (BigInt::from(0)),
                    // reduce other `Addition`s
                    terms => {
                        let mut integer_sum: BigInt = BigInt::ZERO;
                        let mut other_terms: Vec<Expression> = Vec::new();
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
            Subtraction (terms) => {
                let left: Expression = terms.0.reduce();
                let right: Expression = terms.1.reduce();
                match (&left, &right) {
                    // subtract integers
                    (Integer (left), Integer(right)) => Integer (left - right),
                    _ => Subtraction (Box::new((left, right))),
                }
            }
            Multiplication (factors) => {
                let factors: Vec<Self> = factors.into_iter()
                    .map(|factor| factor.reduce())
                    .collect();
                match factors {
                    // remove unnecessary `Multiplication` from single term
                    factors if factors.len() == 1 => factors[0].clone(),
                    // convert empty `Multiplication` to `0`
                    factors if factors.len() == 0 => Integer (BigInt::from(0)),
                    // reduce other `Multiplication`s
                    terms => {
                        let mut integer_product: BigInt = BigInt::from(1);
                        let mut other_terms: Vec<Expression> = Vec::new();
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
                let dividend: Expression = terms.0.reduce();
                let divisor: Expression = terms.1.reduce();
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
                let base: Expression = terms.0.reduce();
                let exponent: Expression = terms.1.reduce();
                Power (Box::new((base, exponent)))
            }
            other => other
        }
    }

    /// Differentiates this `Expression` with respect to a variable
    pub fn differentiate(&self, variable: &str) -> Self {
        use Expression::*;
        match self {
            // identity rule
            Variable (name) if name == variable => Integer (BigInt::from(1)),
            // variable rule
            Variable (_) => Integer (BigInt::from(0)),
            // constant rule
            Integer (_) => Integer (BigInt::from(0)),
            // sum rule
            Addition (terms) => Addition (terms.iter()
                .map(|operand| operand.differentiate(variable))
                .collect()
            ),
            // difference rule
            Subtraction (terms) => Subtraction (Box::new((
                terms.0.differentiate(variable), terms.1.differentiate(variable)
            ))),
            // product rule
            Multiplication (factors) => Addition (factors.iter()
                .enumerate()
                .map(|(factor_index, factor)| {
                    let mut output: Vec<Expression> = Vec::with_capacity(factors.len());
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
                Subtraction (Box::new((
                    Multiplication (vec![terms.0.differentiate(variable), terms.1.clone()]),
                    Multiplication (vec![terms.0.clone(), terms.1.differentiate(variable)]),
                ))),
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
        }.reduce()
    }

}