// Copyright Rob Gage 2025

use num_bigint::BigInt;

/// An algebraic expression
#[derive(Clone, PartialEq)]
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

    /// Reduce an `Expression`, or returns it unchanged if not reducible
    fn reduce(self) -> Self {
        todo!()
    }

    /// Differentiates this `Expression` with respect to a variable
    pub fn differentiate(&self, variable: &str) -> Self {
        use Expression::*;
        let reduced: Self = self.clone().reduce();
        match self {
            Variable (name) if name == variable => Integer (BigInt::from(1)),
            Variable (_) => Integer (BigInt::from(0)),
            Integer (_) => Integer (BigInt::from(0)),
            Addition (operands) => Addition (operands.iter()
                .map(|operand| operand.differentiate(variable))
                .collect()
            ),
            Subtraction (operands) => Subtraction (Box::new((
                operands.0.differentiate(variable), operands.1.differentiate(variable)
            ))),
            Multiplication (operands) => Addition (operands.iter()
                .enumerate()
                .map(|(factor_index, factor)| {
                    let mut output: Vec<Expression> = Vec::with_capacity(operands.len());
                    output.push(factor.differentiate(variable));
                    for index in 0..operands.len() {
                        if index != factor_index {
                            output.push(operands[index].clone());
                        }
                    }
                    Multiplication (output)
                })
                .collect()
            ),
            Division (operands) => Division (Box::new((
                Subtraction (Box::new((
                    Multiplication (vec![operands.0.differentiate(variable), operands.1.clone()]),
                    Multiplication (vec![operands.0.clone(), operands.1.differentiate(variable)]),
                ))),
                Multiplication (vec![operands.1.clone(), operands.1.clone()])
            )))
        }.reduce()
    }

}