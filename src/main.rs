mod operation;
mod parser;
mod evaluator;

use std::io;
use parser::{parse_expression, to_rpn};
use evaluator::evaluate_rpn;

fn main() {
    println!("Enter an expression: ");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read line");

    match parse_expression(expression.trim()) {
        Ok(tokens) => {
            let rpn = to_rpn(tokens);
            match evaluate_rpn(rpn) {
                Some(result) => println!("Result: {}", result),
                None => println!("Error: Division by zero"),
            }
        }
        Err(e) => println!("Error parsing expression: {}", e),
    }
}
