// Copyright Rob Gage 2025

use crate::Expression::{
    *,
    self,
};
use num::{
    bigint::BigInt,
    rational::BigRational,
};
use std::{
    collections::HashMap,
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
            match factor {
                Product (factors) => monomials.push(Self::from_factors(factors)),
                Quotient (operands) => match (&operands.0, &operands.1) {
                    (Integer (numerator), Integer (denominator)) =>
                        multiplier *= BigRational::new(numerator.clone(), denominator.clone()),
                    _ => other_factors.push(Quotient (operands.clone())),
                }
                Power (operands) => match (&operands.0, &operands.1) {
                    (Variable (identifier), Integer (exponent)) =>
                        if variables.contains_key(identifier) {
                            *variables.get_mut(identifier).unwrap() += exponent;
                        } else { variables.insert(identifier.clone(), exponent.clone()); },
                    _ => other_factors.push(Power (operands.clone())),
                }
                Integer (integer) => multiplier *= BigRational::from_integer(integer.clone()),
                Variable (identifier) => if variables.contains_key(identifier) {
                    *variables.get_mut(identifier).unwrap() += BigInt::from(1);
                } else { variables.insert(identifier.clone(), BigInt::from(1)); },
                other => other_factors.push(other.clone()),
            }
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

    /// Returns a `Monomial` as its factors
    pub fn to_factors(self) -> Vec<Expression<I>> {
        let mut factors: Vec<Expression<I>> = self.variables.into_iter()
            .map(|(identifier, exponent)| if exponent == BigInt::from(1) {
                Variable(identifier)
            } else { Power (Box::new ((
                Variable (identifier),
                Integer (exponent)
            ))) })
            .collect();
        factors.extend(self.other_factors.into_iter());
        factors.push(if self.multiplier.is_integer() {
            Integer (self.multiplier.to_integer())
        } else {
            let (numerator, denominator): (BigInt, BigInt) = self.multiplier.into_raw();
            Quotient (Box::new ((Integer (numerator), Integer(denominator))))
        });
        factors
    }

}