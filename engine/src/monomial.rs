// Copyright Rob Gage 2025

use crate::Expression::{
    *,
    self,
};
use num::{
    bigint::BigInt,
    rational::BigRational,
    One,
    Zero
};
use std::{
    collections::HashMap,
    fmt::{
        Display,
        Formatter,
        Result as FormatResult,
    },
    hash::Hash,
};

/// A monomial
pub struct Monomial<I: Clone + Eq + Hash + PartialEq> {
    /// A scalar
    multiplier: BigRational,
    /// Variables with exponents
    variables: HashMap<I, BigInt>,
    /// Other factors
    other_factors: Vec<Expression<I>>,
}

impl<I: Clone + Eq + Hash + PartialEq> Monomial<I> {

    /// Creates a new `Monomial` from factors
    pub fn from_factors(factors: &[Expression<I>]) -> Self {
        let mut multiplier: BigRational = BigRational::from(BigInt::from(1));
        let mut variables: HashMap<I, BigInt> = HashMap::new();
        let mut other_factors: Vec<Expression<I>> = vec![];
        let mut monomials: Vec<Self> = Vec::new();
        for factor in factors {
            match factor.clone().reduce() {
                Product (factors) => monomials.push(Self::from_factors(&factors)),
                Quotient (operands) => match (operands.0, operands.1) {
                    (Integer (numerator), Integer (denominator)) =>
                        multiplier *= BigRational::new(numerator.clone(), denominator.clone()),
                    operands => other_factors.push(Quotient (Box::new(operands))),
                }
                Power (operands) => match (operands.0, operands.1) {
                    (Variable (identifier), Integer (exponent)) =>
                        if variables.contains_key(&identifier) {
                            *variables.get_mut(&identifier).unwrap() += exponent;
                        } else { variables.insert(identifier, exponent); },
                    operands => other_factors.push(Power (Box::new(operands))),
                }
                Integer (integer) => multiplier *= BigRational::from_integer(integer.clone()),
                Variable (identifier) => if variables.contains_key(&identifier) {
                    *variables.get_mut(&identifier).unwrap() += BigInt::from(1);
                } else { variables.insert(identifier, BigInt::from(1)); },
                other => other_factors.push(other),
            }
            if multiplier.is_zero() { return Monomial {
                multiplier: BigRational::zero(),
                variables: HashMap::new(),
                other_factors: vec![],
            }}
        }
        for monomial in monomials {
            multiplier *= monomial.multiplier;
            other_factors.extend(monomial.other_factors);
            for variable in monomial.variables {
                if variables.contains_key(&variable.0) {
                    *variables.get_mut(&variable.0).unwrap() += variable.1;
                } else { variables.insert(variable.0, variable.1); }
            }
        }
        Self { multiplier, variables, other_factors }
    }

    /// Returns a `Monomial` as an expression
    pub fn to_expression(self) -> Expression<I> {
        let factors: Vec<Expression<I>> = self.to_factors();
        if factors.len() == 1 {
            factors[0].clone()
        } else { Product (factors) }
    }

    /// Returns a `Monomial` as its factors
    fn to_factors(self) -> Vec<Expression<I>> {
        let mut factors: Vec<Expression<I>> = self.variables.into_iter()
            .map(|(identifier, exponent)| if exponent == BigInt::from(1) {
                Variable(identifier)
            } else { Power (Box::new ((
                Variable (identifier),
                Integer (exponent)
            ))) })
            .collect();
        factors.extend(self.other_factors.into_iter());
        if self.multiplier.is_integer() {
            if self.multiplier != BigRational::one() {
                factors.push(Integer (self.multiplier.to_integer()));
            }
        } else {
            let (numerator, denominator): (BigInt, BigInt) = self.multiplier.into_raw();
            factors.push(Quotient (Box::new ((Integer (numerator), Integer(denominator)))));
        };
        factors
    }

}

impl Display for Monomial<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        use Expression::*;
        if self.multiplier.is_integer() {
            if self.multiplier != BigRational::one() {
                write!(f, "{}", self.multiplier)?;
            }
        } else { write!(
            f, "\\displaystyle \\frac{{{}}}{{{}}}",
            self.multiplier.numer(), self.multiplier.denom())
        ? }
        let mut variables: Vec<(&String, &BigInt)> = self.variables.iter().collect();
        variables.sort_by(|(a_name, a_exponent), (b_name, b_exponent)| {
            match b_exponent.cmp(a_exponent) {
                std::cmp::Ordering::Equal => a_name.cmp(b_name),
                other => other,
            }
        });
        for (name, exponent) in variables {
            if exponent != &BigInt::from(1) {
                write!(f, "\\mathit{{{}}}^{{{}}}", name, exponent)?;
            } else { write!(f, "\\mathit{{{}}}", name)?; }
        }
        for factor in self.other_factors.iter() {
            write!(f, "\\left({}\\right)", factor)?;
        }
        Ok (())
    }
}