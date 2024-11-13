## Rust Calculator

This project is a command-line calculator built in Rust. The calculator supports basic arithmetic operations and evaluates expressions with operator precedence, functioning similarly to a standard calculator.

### Features

- **Basic Operations**: Perform addition, subtraction, multiplication, and division.
- **Operator Precedence**: Supports operator precedence, allowing accurate calculations for expressions like `3 + 5 * 2`.
- **Expression Parsing**: Parses expressions from user input and converts them into Reverse Polish Notation (RPN) for efficient evaluation.
- **Error Handling**: Detects and handles division by zero and invalid inputs.

### Project Structure

- **operation.rs**: Defines the `Operation` enum and implements functions for precedence and operation application.
- **parser.rs**: Contains functions for parsing expressions and converting them to RPN.
- **evaluator.rs**: Implements the RPN evaluation logic to compute the final result.
- **main.rs**: Handles user input and integrates parsing, RPN conversion, and evaluation for result output.

### Usage

1. Clone the repository and navigate to the project directory.
2. Run the following command to start the calculator:
   ```bash
   cargo run
