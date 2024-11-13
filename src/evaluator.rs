use crate::parser::Token;

pub fn evaluate_rpn(tokens: Vec<Token>) -> Option<f64> {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => stack.push(num),
            Token::Operator(op) => {
                if let (Some(num2), Some(num1)) = (stack.pop(), stack.pop()) {
                    if let Some(result) = op.apply(num1, num2) {
                        stack.push(result);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }
    }
    stack.pop()
}
