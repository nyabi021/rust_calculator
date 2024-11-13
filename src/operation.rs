#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn precedence(&self) -> u8 {
        match self {
            Operation::Add | Operation::Subtract => 1,
            Operation::Multiply | Operation::Divide => 2,
        }
    }

    pub fn apply(&self, num1: f64, num2: f64) -> Option<f64> {
        match self {
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
}
