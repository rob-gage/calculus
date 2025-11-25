// Copyright Rob Gage 2025

use crate::Expression;
use std::collections::HashMap;

/// Associates variables and numeric identifiers
pub struct Namespace {
    /// The variables in the namespace
    variables: Vec<String>,
    identifiers: HashMap<String, usize>,
}

impl Namespace {

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            identifiers: HashMap::new(),
        }
    }

    /// Converts an `Expression` with `String` identifiers into one with `usize` identifiers,
    /// and stores the `String` identifiers
    pub fn intern(&mut self, expression: Expression<String>) -> Expression {
        use Expression::*;
        match expression {
            Addition (terms) => Addition (terms.into_iter()
                .map(|term| self.intern(term))
                .collect()
            ),
            Multiplication (factors) => Multiplication (factors.into_iter()
                .map(|factor| self.intern(factor))
                .collect()
            ),
            Division (operands) => Division (Box::new((
                self.intern(operands.0),
                self.intern(operands.1)
            ))),
            Power (operands) => Power (Box::new((
                self.intern(operands.0),
                self.intern(operands.1)
            ))),
            Exponential (operand) => Exponential (Box::new(self.intern(*operand))),
            Logarithm (operand) => Exponential (Box::new(self.intern(*operand))),
            Variable (name) => if let Some (identifier) = self.identifiers.get(&name) {
                Variable (*identifier)
            } else {
                let identifier: usize = self.variables.len();
                self.identifiers.insert(name.clone(), identifier);
                self.variables.push(name.clone());
                Variable (identifier)
            }
            Integer (integer) => Integer (integer)
        }
    }

    /// Displays an expression
    pub fn display(&self)


}