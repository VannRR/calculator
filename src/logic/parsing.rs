use super::arithmetic::*;
use crate::types::Int;

// Define supported operators and their precedence and associativity
// (operator, precedence, associativity)
const OPERATORS: [(&str, u8, u8); 6] = [
    ("^", 5, 1),
    ("√", 4, 1),
    ("x", 3, 0),
    ("÷", 3, 0),
    ("+", 2, 0),
    ("-", 2, 0),
];

// Get precedence of a given operator
fn get_op_precedence(op: &str) -> u8 {
    for operator in OPERATORS {
        if operator.0 == op {
            return operator.1;
        }
    }
    panic!("Operator not found");
}

// Get associativity of a given operator
fn get_op_associativity(op: &str) -> u8 {
    for operator in OPERATORS {
        if operator.0 == op {
            return operator.2;
        }
    }
    panic!("Operator not found");
}

// Get result of a given operator on two numbers
fn get_op_result(op: &str, a: Int, b: Int) -> Result<Int, bool> {
    // Check if the given operator is a supported one
    if OPERATORS[0].0 == op {
        return Ok(power(a, b));
    } else if OPERATORS[1].0 == op {
        return Ok(square_root(b));
    } else if OPERATORS[2].0 == op {
        return Ok(multiply(a, b));
    } else if OPERATORS[3].0 == op {
        return Ok(divide(a, b));
    } else if OPERATORS[4].0 == op {
        return Ok(add(a, b));
    } else if OPERATORS[5].0 == op {
        return Ok(subtract(a, b));
    }
    // Otherwise, return a error
    Err(false)
}

// Check if a string is a number
pub fn string_is_number(s: &str) -> bool {
    return s.chars().all(char::is_numeric)
        || s.len() > 1 && s.starts_with('-') && s.chars().skip(1).all(char::is_numeric);
}

// Convert an infix expression to Reverse Polish Notation (RPN) using the shunting-yard algorithm
fn shunting_yard(input: Vec<String>) -> Vec<String> {
    // Initialize output queue and operator stack
    let mut output_queue: Vec<String> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    // clone the input vector
    let infix: Vec<String> = input.clone();

    // Iterate over each token
    for token in infix {
        // Check if token is a number
        if string_is_number(&token) {
            output_queue.push(token);
        } else if token == "(" {
            // If token is an opening parenthesis, push it onto the operator stack
            operator_stack.push(token);
        } else if token == ")" {
            // If token is a closing parenthesis, pop operators from operator stack
            // and push them onto output queue until an opening parenthesis is encountered
            while let Some(op) = operator_stack.pop() {
                if op == "(" {
                    break;
                } else {
                    output_queue.push(op);
                }
            }
        } else {
            // If token is an operator
            let o1 = token.clone();
            if let Some(last) = operator_stack.last() {
                let mut o2 = last.clone();
                // While there are operators on the operator stack with higher precedence than o1
                while o2 != "("
                    && ((get_op_associativity(&o1) == 0
                        && get_op_precedence(&o1) <= get_op_precedence(&o2))
                        || (get_op_associativity(&o1) == 1
                            && get_op_precedence(&o1) < get_op_precedence(&o2)))
                {
                    // Pop them from operator stack and push them onto output queue
                    output_queue.push(operator_stack.pop().unwrap());
                    if let Some(new_last) = operator_stack.last() {
                        o2 = new_last.clone();
                    } else {
                        break;
                    }
                }
            }
            // Push o1 onto operator stack
            operator_stack.push(o1);
        }
    }

    // Pop any remaining operators from operator stack and push them onto output queue
    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    // Return contents of output queue as result
    output_queue
}

// Evaluate a mathematical expression in Reverse Polish Notation (RPN)
fn evaluate_rpn(rpn: Vec<String>) -> Result<Int, bool> {
    if rpn.len() < 2 {
        return Err(false);
    }
    // Initialize stack to hold intermediate results
    let mut stack: Vec<Int> = Vec::new();
    // Iterate over each token in the RPN expression
    for token in rpn {
        // Check if token is a number
        if string_is_number(&token) {
            if token.starts_with('-') && token.parse::<Int>().is_err() {
                // If token is a negative number and cannot be parsed as an Int, push Int::MIN
                stack.push(Int::MIN);
            } else if token.parse::<Int>().is_err() {
                // If token is a positive number and cannot be parsed as an Int, push Int::MAX
                stack.push(Int::MAX);
            } else {
                // Otherwise, push token as an Int onto the stack
                stack.push(token.parse::<Int>().unwrap());
            }
        } else {
            // If token is an operator, pop two values from the stack
            let b = stack.pop().unwrap_or(0);
            let a = stack.pop().unwrap_or(0);

            // Calculate the result of the operation
            let result = get_op_result(&token, a, b);

            if result == Err(false) {
                return Err(false);
            }

            // Push the result back onto the stack
            stack.push(result.unwrap());
        }
    }
    // Pop final result from stack and return it
    Ok(stack.pop().unwrap())
}

// Calculate the result of a mathematical expression in infix notation
pub fn calculate(input: Vec<String>, error_msg: &str) -> String {
    // Check for special cases
    if input.is_empty() {
        // If empty, return empty string
        return "".to_string();
    } else if input.len() == 1 {
        // If input contains one item, return the first item
        return input.first().unwrap().to_string();
    }

    // Convert infix expression to RPN using shunting-yard algorithm
    let rpn = shunting_yard(input);
    // Evaluate RPN expression
    let result = evaluate_rpn(rpn);

    // Check if result is an error
    if result == Err(false) {
        // If it is, return the error message
        error_msg.to_string()
    } else {
        // Otherwise, return the result
        result.unwrap().to_string()
    }
}
