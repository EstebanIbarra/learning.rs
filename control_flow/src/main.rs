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
        is_data_type(input, false);
        println!();
    }
}

fn parse_value(input: &str) -> Value {
    serde_json::from_str(input)
        .expect("This is not an array")
}

fn print_result(input: &str, data_type: &str, is_array_value: bool) {
    if is_array_value {
        println!("    containing the {}: {}", data_type, input);
        return;
    }
    println!("You prompted the {}: {}", data_type, input);
}

fn parse_array(input: &str) {
    match parse_value(input).as_array() {
        Some(array) => {
            print_result(input, "array", false);
            print_array(array);
        },
        None => print_result(input, "array", false),
    }
}

fn print_array(array: &Vec<Value>) {
    for value in array {
        match value.as_str() {
            Some(string) => is_data_type(string, true),
            None => is_data_type(value.to_string().trim(), true),
        }
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

fn match_null_string_or_char(input: &str, is_array_value: bool) {
    match input.len().cmp(&1) {
        Ordering::Less => print_result("null", "null value", is_array_value),
        Ordering::Equal => print_result(input, "char", is_array_value),
        Ordering::Greater => print_result(input, "string", is_array_value),
    }
}

fn is_data_type(input: &str, is_array_value: bool) {
        if is_number(input) {
            print_result(input, "number", is_array_value);
            return;
        }
        if is_bool(input) {
            print_result(input, "bool", is_array_value);
            return;
        }
        if is_array(input) {
            parse_array(input);
            return;
        }
        match_null_string_or_char(input, is_array_value);
}
