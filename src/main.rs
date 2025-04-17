use std::{io, process};
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
        let op = get_input(format!(
        "Please select an operation ({}, {}, {}, {}) or {} to quit:",
            "add".green().bold(),
            "sub".yellow().bold(),
            "mul".cyan().bold(),
            "div".magenta().bold(),
            "q".red().bold()
        ).as_str());
        if op.eq_ignore_ascii_case("q") {
            process::exit(0)
        }
        let op = match parse_operation(&op) {
            Some(op) => op,
            None => {
                println!("{}", "Invalid opperation".bold().red());
                continue;
            }
        };

        let num1 = get_input("Please type the first number or 'q' to quit:");
        if num1.eq_ignore_ascii_case("q") {
            process::exit(0)
        }
        let num1 = match parse_number(&num1) {
            Some(num) => num,
            None => {
                println!("{}", "Invalid number".bold().red());
                continue;
            }
        };

        let num2 = get_input("Please type the seccond number or 'q' to quit:");
        if num2.eq_ignore_ascii_case("q") {
            process::exit(0)
        }
        let num2 = match parse_number(&num2) {
            Some(num) => num,
            None => {
                println!("{}", "Invalid number".bold().red());
                continue;
            }
        };

        let result = match compute(op, num1, num2) {
            Some(result) => result,
            None => {
                println!("{}", "Cannot divide by zero.".bold().red());
                continue;
            }
        };
        let result_str = format!("The result is {:.3}", result);
        println!("{}", result_str.bright_green().bold().italic());

        if ask_yes_no("Would you like to see the precise answer? [Y/n]:") {
            println!("{}", format!("The precise result is {:.15}", result).bright_green().bold())
        }
    }
}

fn ask_yes_no(prompt: &str) -> bool {
    loop {
        match get_input(prompt).to_lowercase().as_str() {
            "y" => return true,
            "n" | "" => return false,
            _ => {
                println!("{}", "Please select either 'y' or 'n'".bold().red())
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt.blue());
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn compute(op: Operation, num1: f64, num2: f64) -> Option<f64> {
    match op {
        Operation::Addition => Some(num1 + num2),
        Operation::Subtraction => Some(num1 - num2),
        Operation::Multiplication => Some(num1 * num2),
        Operation::Division => {
            if num2 == 0.0 {
                None
            } else{
                Some(num1 / num2)
            }   
        }
    }
}

fn parse_operation(input: &str) -> Option<Operation> {
    match input {
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
