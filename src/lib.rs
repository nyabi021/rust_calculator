enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn calculate(operation: Operation, num1: f64, num2: f64) -> Option<f64> {
    match operation{
        Operation::Add => Some(num1 + num2),
        Operation::Subtract => Some(num1 - num2),
        Operation::Multiply => Some(num1 * num2),
        Operation::Divide => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                None
            }
        }
    }
}
