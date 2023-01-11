/*
Calculator app using Rust
Get number1, operator, number2 from cli
Parse number1, number2 to float64
Parse operator to char
Calculate the values based on operator
*/

// std:env is used to get args from cli
use std::env::{args, Args};

// main function
fn main() {
    let mut args: Args = args(); // returns the arguments passed
    let first_number = args.nth(1).unwrap(); // get the first argument(num1)
                                             //args.nth() -> returns iterator. So use (0) to get next value
    let operator = args.nth(0).unwrap().chars().next().unwrap(); // Get the operator and parse it to char
    let second_number = args.nth(0).unwrap(); // get the third argument(num2)

    // parse to f64
    let first_number = first_number.parse::<f64>().unwrap();
    let second_number = second_number.parse::<f64>().unwrap();

    let result = calculate1(first_number, second_number, operator);
    println!(
        "Method 1 - {}{}{}={}",
        first_number, operator, second_number, result
    );

    let result = calculate2(first_number, second_number, operator);
    println!(
        "Method 2 - {}{}{}={}",
        first_number, operator, second_number, result
    );
}

// calculator1 function
fn calculate1(num1: f64, num2: f64, operator: char) -> f64 {
    //Method 1
    if operator == '+' {
        return num1 + num2;
    } else if operator == '-' {
        return num1 - num2;
    } else if operator == 'x' {
        return num1 * num2;
    } else if operator == '/' {
        return num1 / num2;
    } else {
        return 0.0;
    }
}

// calculator2 function
fn calculate2(num1: f64, num2: f64, operator: char) -> f64 {
    //Method 1
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        'x' => num1 * num2,
        '/' => num1 / num2,
        _ => 0.0,
    }
}
