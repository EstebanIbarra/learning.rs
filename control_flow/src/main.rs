use std::io;
use std::cmp::Ordering;
use serde_json::Value;

fn main() {
    println!("Data Type Detector");
    println!("Type \"exit\" to end the program\n");
    loop {
        println!("Please input something\n");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        println!();
        if input == "exit" {
            println!("Bye!");
            break;
        }
        if is_number(input) {
            print_result(input, "number", false);
            println!();
            continue;
        }
        if is_bool(input) {
            print_result(input, "bool", false);
            println!();
            continue;
        }
        if is_array(input) {
            print_result(input, "array", false);
            let value: Value = parse_value(input);
            print_array(value.as_array().unwrap());
            println!();
            continue;
        }
        match_null_string_or_char(input, false);
    }
}

fn print_result(input: &str, data_type: &str, is_array_value: bool) {
    if is_array_value {
        println!("    containing the {}: {}", data_type, input);
        return;
    }
    println!("You prompted the {}: {}", data_type, input);
}

fn print_array(array: &Vec<Value>) {
    for value in array {
        let value_string: &str = value.as_str().unwrap().trim();
        if is_number(value_string) {
            print_result(value_string, "number", true);
            continue;
        }
        if is_bool(value_string) {
            print_result(value_string, "bool", true);
            continue;
        }
        if is_array(value_string) {
            print_result(value_string, "array", false);
            let value: Value = parse_value(value_string);
            print_array(value.as_array().unwrap());
            continue;
        }
        match_null_string_or_char(value_string, true);
    }
}

fn is_number(input: &str) -> bool {
    match input.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_bool(input: &str) -> bool {
    match input.parse::<bool>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_array(input: &str) -> bool {
    match serde_json::from_str::<Value>(input) {
        Ok(value) => value.is_array(),
        Err(_) => false,
    }
}

fn parse_value(input: &str) -> Value {
    serde_json::from_str(input)
        .expect("This is not an array")
}

fn match_null_string_or_char(input: &str, is_array_value: bool) {
    match input.len().cmp(&1) {
        Ordering::Less => {
            print_result("null", "null value", is_array_value);
            if !is_array_value {
                println!();
            }
        },
        Ordering::Equal => {
            print_result(input, "char", is_array_value);
            if !is_array_value {
                println!();
            }
        },
        Ordering::Greater => {
            print_result(input, "string", is_array_value);
            if !is_array_value {
                println!();
            }
        },
    }
}
