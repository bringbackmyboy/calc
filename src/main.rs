use std::io;
use colored::*;

enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division
}

fn main() {
    println!("{}", r"
 _____   ___   _      _____  _   _  _       ___   _____  _____ ______ 
/  __ \ / _ \ | |    /  __ \| | | || |     / _ \ |_   _||  _  || ___ \
| /  \// /_\ \| |    | /  \/| | | || |    / /_\ \  | |  | | | || |_/ /
| |    |  _  || |    | |    | | | || |    |  _  |  | |  | | | ||    / 
| \__/\| | | || |____| \__/\| |_| || |____| | | |  | |  \ \_/ /| |\ \ 
 \____/\_| |_/\_____/ \____/ \___/ \_____/\_| |_/  \_/   \___/ \_| \_|".bold().green());
    loop {
        let mut op = String::new();
        let mut num1 = String::new();
        let mut num2 = String::new();

        println!("{}", "Please select an operation ('add', 'sub', 'mul', 'div'):".blue());
        io::stdin().read_line(&mut op).expect("Failed to read line");

        let op = match parse_operation(&op) {
            Some(op) => op,
            None => {
                println!("{}", "Invalid opperation".bold().red());
                continue;
            }
        };

        println!("{}", "Plese type the first number:  ".blue());
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        
        let num1 = match parse_number(num1.trim()) {
            Some(num) => num,
            None => {
                println!("{}", "Invalid number".bold().red());
                continue;
            }
        };

        println!("{}", "Plese type the seccond number:  ".blue());
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        
        let num2 = match parse_number(num2.trim()) {
            Some(num) => num,
            None => {
                println!("{}", "Invalid number".bold().red());
                continue;
            }
        };

        let result = compute(op, num1, num2);
        let result_str = format!("The result is {}", result);
        println!("{}", result_str.bright_green().bold().italic());
    }
}

fn compute(op: Operation, num1: f64, num2: f64) -> f64 {
    match op {
        Operation::Addition => num1 + num2,
        Operation::Subtraction => num1 - num2,
        Operation::Multiplication => num1 * num2,
        Operation::Division => num1 / num2
    }
}

fn parse_operation(input: &str) -> Option<Operation> {
    match input.trim().to_lowercase().as_str() {
        "add" => Some(Operation::Addition),
        "sub" => Some(Operation::Subtraction),
        "mul" => Some(Operation::Multiplication),
        "div" => Some(Operation::Division),
        _ => None
    }
}

fn parse_number(input: &str) -> Option<f64> {
    match input.parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
