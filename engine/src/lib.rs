// Copyright Rob Gage 2025

mod expression;
mod namespace;
mod monomial;

pub use expression::Expression;
pub type Syntax = Expression<String>;