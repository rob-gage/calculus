// Copyright Rob Gage 2025

use core::Expression;
use syntax::parse_expression;
use std::io::{
    stdin,
    stdout,
    Write,
};

fn main() {
    loop {
        print!("differentiate expression: ");
        stdout().flush().unwrap();
        let mut expression_string: String = String::new();
        stdin().read_line(&mut expression_string).unwrap();
        print!("with respect to variable: ");
        stdout().flush().unwrap();
        let mut variable_string: String = String::new();
        stdin().read_line(&mut variable_string).unwrap();
        match parse_expression(&expression_string) {
            Ok (expression) => {
                println!("\nParsed: {}\n", expression);
                println!("Differentiated: {}\n\n", expression.differentiate(variable_string.trim()));
            }
            Err (_) => println!("\nInvalid expression\n\n"),
        };
    }
}