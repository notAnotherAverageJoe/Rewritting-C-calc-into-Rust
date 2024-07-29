use std::io;
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Power,
}

fn read_number(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input.trim().parse().expect("Invalid number")
}

fn read_operation() -> Operation {
    let mut input = String::new();
    println!("Enter the operation (+, -, *, /, %, ^): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        "%" => Operation::Remainder,
        "^" => Operation::Power,
        _ => panic!("Invalid operation!"),
    }
}

fn main() {
    let num1 = read_number("Enter the first number:");
    let op = read_operation();
    let num2 = read_number("Enter the second number:");

    let result = match op {
        Operation::Add => num1 + num2,
        Operation::Subtract => num1 - num2,
        Operation::Multiply => num1 * num2,
        Operation::Divide => {
            if num2 == 0.0 {
                panic!("Cannot divide by zero!");
            }
            num1 / num2
        }
        Operation::Remainder => {
            if num2 == 0.0 {
                panic!("Cannot find remainder with divisor zero!");
            }
            num1 % num2
        }
        Operation::Power => num1.powf(num2),
    };

    println!("Result: {}", result);
}
