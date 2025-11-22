// Copyright Rob Gage 2025

use num_bigint::BigInt;
use num_integer::Integer;

/// An algebraic expression
#[derive(Clone, Eq, PartialEq)]
pub enum Expression {
    /// Addition of subexpressions
    Addition (Vec<Expression>),
    /// Subtraction of subexpressions
    Subtraction (Box<(Expression, Expression)>),
    /// Multiplication of subexpressions
    Multiplication (Vec<Expression>),
    /// Division of subexpressions
    Division (Box<(Expression, Expression)>),
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
            Addition (terms) => match terms {
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
            Subtraction (terms) => match *terms {
                // difference is zero
                (left, right) if left == right => Integer (BigInt::ZERO),
                // subtract integers
                (Integer (left), Integer(right)) => Integer (left - right),
                _ => Subtraction (terms)
            }
            Multiplication (factors) => match factors {
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
            Division (terms) => match *terms {
                // reduce fractions
                (Integer (numerator), Integer (denominator)) => {
                    let gcd: BigInt = numerator.gcd(&denominator);
                    let numerator: BigInt = numerator / &gcd;
                    let denominator: BigInt = denominator / &gcd;
                    if denominator == BigInt::from(1) { Integer (numerator) } else {
                        Division (Box::new((Integer (numerator), Integer (denominator))))
                    }
                }
                _ => Division (terms)
            }
            other => other
        }
    }

    /// Differentiates this `Expression` with respect to a variable
    pub fn differentiate(&self, variable: &str) -> Self {
        use Expression::*;
        let reduced: Self = self.clone().reduce();
        match self {
            Variable (name) if name == variable => Integer (BigInt::from(1)),
            Variable (_) => Integer (BigInt::from(0)),
            Integer (_) => Integer (BigInt::from(0)),
            Addition (terms) => Addition (terms.iter()
                .map(|operand| operand.differentiate(variable))
                .collect()
            ),
            Subtraction (terms) => Subtraction (Box::new((
                terms.0.differentiate(variable), terms.1.differentiate(variable)
            ))),
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
            Division (terms) => Division (Box::new((
                Subtraction (Box::new((
                    Multiplication (vec![terms.0.differentiate(variable), terms.1.clone()]),
                    Multiplication (vec![terms.0.clone(), terms.1.differentiate(variable)]),
                ))),
                Multiplication (vec![terms.1.clone(), terms.1.clone()])
            )))
        }.reduce()
    }

}