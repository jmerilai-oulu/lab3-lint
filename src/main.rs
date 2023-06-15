use std::io::{self, Write};

fn AskOperator() -> char {
    let mut operator = String::new();

    print!("Enter an operator (+, -, *, /): ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    match operator.trim() {
        "+" | "-" | "*" | "/" => operator.trim().chars().next().unwrap(),
        _ => {
            println!("Invalid operator. Please try again.");
            AskOperator()
        }
    }
}

fn Add(x: i32, y: i32) -> i32 {
    return x + y
}

fn Subtract(x: i32, y: i32) -> i32 {
    return x - y
}

fn Multiply(x: i32, y: i32) -> i32 {
    return x * y
}

fn Divide(x: i32, y: i32) -> Option<i32> {
    if y != 0 {
        return Some(x / y)
    } else {
        return None
    }
}

fn main() {
    let OPERATOR = AskOperator();

    print!("Enter the first number: ");
    io::stdout().flush();
    let mut x_input = String::new();
    io::stdin()
        .read_line(&mut x_input)
        .expect("Failed to read line");
    let x: i32 = x_input.trim().parse().expect("Invalid number");

    print!("Enter the second number: ");
    io::stdout().flush();
    let mut y_input = String::new();
    io::stdin()
        .read_line(&mut y_input)
        .expect("Failed to read line");
    let y: i32 = y_input.trim().parse().expect("Invalid number");

    let result = match OPERATOR {
        '+' => Add(x, y),
        '-' => Subtract(x, y),
        '*' => Multiply(x, y),
        '/' => match Divide(x, y) {
            Some(value) => value,
            None => {
                println!("Division by zero is not allowed");
                return;
            }
        },
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
}
