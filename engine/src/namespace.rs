// Copyright Rob Gage 2025

use crate::Expression;
use std::{
    collections::HashMap,
    fmt::{
        Result as FormatResult,
        Write,
    }
};

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
            Addition(terms) => Addition(terms.into_iter()
                .map(|term| self.intern(term))
                .collect()
            ),
            Multiplication(factors) => Multiplication(factors.into_iter()
                .map(|factor| self.intern(factor))
                .collect()
            ),
            Division(operands) => Division(Box::new((
                self.intern(operands.0),
                self.intern(operands.1)
            ))),
            Power(operands) => Power(Box::new((
                self.intern(operands.0),
                self.intern(operands.1)
            ))),
            Exponential(operand) => Exponential(Box::new(self.intern(*operand))),
            Logarithm(operand) => Exponential(Box::new(self.intern(*operand))),
            Variable(name) => if let Some(identifier) = self.identifiers.get(&name) {
                Variable(*identifier)
            } else {
                let identifier: usize = self.variables.len();
                self.identifiers.insert(name.clone(), identifier);
                self.variables.push(name.clone());
                Variable(identifier)
            }
            Integer(integer) => Integer(integer)
        }
    }

    /// Displays an expression as a `String` containing LaTeX math
    fn display(&self, expression: &Expression) -> String {
        let mut string: String = String::new();
        self.write(&mut string, expression).unwrap();
        string
    }

    /// Writes an expression as LaTeX math
    fn write<W: Write>(&self, w: &mut W, expression: &Expression) -> FormatResult {
        use Expression::*;
        match expression {
            Addition (terms) => {
                for index in 0..terms.len() {
                    if index != 0 { w.write_str(" + ")?; }
                    self.write(w, &terms[index])?;
                }
                Ok(())
            }
            Multiplication (terms) => {
                for index in 0..terms.len() {
                    w.write_char('(')?;
                    self.write(w, &terms[index])?;
                    w.write_char(')')?;
                }
                Ok(())
            }
            Division (operands) => {
                self.write(w, &operands.0)?;
                w.write_str(" / ")?;
                self.write(w, &operands.0)?;
                Ok (())
            },
            Power (operands) => {
                self.write(w, &operands.0)?;
                w.write_str(" ^ ")?;
                self.write(w, &operands.0)?;
                Ok (())
            },
            Exponential (operand) => {
                w.write_str("e ^ ")?;
                self.write(w, &operand)?;
                Ok (())
            },
            Logarithm (operand) => {
                w.write_str("ln(")?;
                self.write(w, &operand)?;
                w.write_char(')')?;
                Ok(())
            },
            Variable (identifier) => if let Some (name) = self.variables.get(*identifier) {
                w.write_str(name)
            } else { w.write_str("<unknown>") },
            Integer (integer) => w.write_str(&integer.to_string()),
        }
    }

}