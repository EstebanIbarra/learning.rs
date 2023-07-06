use std::io;
use std::cmp::Ordering;
use serde_json::Value;

fn main() {
    println!("Data Type Detector");
    println!("Type \"exit\" to end the program\n");
    loop {
        println!("Please input something");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            println!("Bye!");
            break;
        }
        if is_number(input) {
            print_result(input, "number");
            continue;
        }
        if is_bool(input) {
            print_result(input, "bool");
            continue;
        }
        if is_array(input) {
            let value: Value = parse_value(input);
            print_array(value.as_array().unwrap());
            continue;
        }
        match input.trim().len().cmp(&1) {
            Ordering::Less => print_result("null value", "null"),
            Ordering::Equal => print_result(input, "char"),
            Ordering::Greater => print_result(input, "string"),
        }
    }
}

fn print_result(input: &str, data_type: &str) {
    println!("You prompted the {}: {}\n", data_type, input);
}

fn print_array(array: &Vec<Value>) {
    for value in array {
        let value_string: &str = value.as_str().unwrap();
        if is_number(value_string) {
            print_result(value_string, "number");
            continue;
        }
        if is_bool(value_string) {
            print_result(value_string, "bool");
            continue;
        }
        if is_array(value_string) {
            let value: Value = parse_value(value_string);
            print_array(value.as_array().unwrap());
            continue;
        }
    }
}

fn is_number(input: &str) -> bool {
    match input.trim().parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_bool(input: &str) -> bool {
    match input.trim().parse::<bool>() {
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
