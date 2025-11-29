// Copyright Rob Gage 2025

mod expression;
mod namespace;
mod monomial;

use monomial::Monomial;

pub use expression::Expression;
pub type Syntax = Expression<String>;