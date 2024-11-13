use crate::operation::Operation;

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Operator(Operation),
}

pub fn parse_expression(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut num = String::new();

    for c in expression.chars() {
        if c.is_digit(10) || c == '.' {
            num.push(c);
        } else if "+-*/".contains(c) {
            if !num.is_empty() {
                tokens.push(Token::Number(num.parse().map_err(|_| "Invalid number")?));
                num.clear();
            }
            let operation = match c {
                '+' => Operation::Add,
                '-' => Operation::Subtract,
                '*' => Operation::Multiply,
                '/' => Operation::Divide,
                _ => return Err("Invalid operator".into()),
            };
            tokens.push(Token::Operator(operation));
        } else if !c.is_whitespace() {
            return Err("Invalid character in expression".into());
        }
    }
    if !num.is_empty() {
        tokens.push(Token::Number(num.parse().map_err(|_| "Invalid number")?));
    }

    Ok(tokens)
}

pub fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(&Token::Operator(top_op)) = operators.last() {
                    if top_op.precedence() >= op.precedence() {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(Token::Operator(op));
            }
        }
    }
    while let Some(op) = operators.pop() {
        output.push(op);
    }
    output
}
