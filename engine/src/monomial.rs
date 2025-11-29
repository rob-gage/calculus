// Copyright Rob Gage 2025

use crate::Expression;
use num_bigint::BigInt;
use std::collections::HashMap;

/// A monomial
pub struct Monomial {
    /// A scalar
    scalar: BigInt,
    /// Variables with exponents
    variables: HashMap<String, BigInt>,
    /// Other factors
    other_factors: Vec<Expression<String>>,
}

impl Monomial {

    /// Creates a new `Monomial` from factors
    pub fn from_factors(factors: &[Expression<String>]) -> Self {
        use Expression::*;
        let mut scalar: BigInt = BigInt::from(1);
        let mut variables: HashMap<String, BigInt> = HashMap::new();
        let mut monomials: Vec<Self> = Vec::new();
        let mut other_factors: Vec<Expression<String>> = vec![];
        for factor in factors {
            match factor {
                Product(factors) => monomials.push(Self::from_factors(factors)),
                Power (operands) => match (&operands.0, &operands.1) {
                    (Variable (name), Integer (exponent)) => if variables.contains_key(name) {
                        *variables.get_mut(name).unwrap() += BigInt::from(1);
                    } else { variables.insert(name.to_string(), BigInt::from(1)); },
                    _ => { }
                }
                Integer (integer) => scalar *= integer,
                Variable (name) => if variables.contains_key(name) {
                    *variables.get_mut(name).unwrap() += BigInt::from(1);
                } else { variables.insert(name.to_string(), BigInt::from(1)); },
                other => other_factors.push(other.clone()),
            }
        }
        Self { scalar, variables, other_factors }
    }

}