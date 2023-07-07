use std::io;
use std::f64::INFINITY;
use std::str::Split;

fn main() {
    println!("Fahrenheit-Celsius Converter\n");
    loop {
        println!("Please input the temperature including the unit:");
        println!("    Example: \"123F\"");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        println!();
        if input.is_empty() {
            println!("No input provided!\n");
            continue;
        }
        if input == "exit" {
            println!("Bye!");
            break;
        }
        let unit: char = input.chars().last().unwrap();
        let input_split: Split<_> = input.split(unit);
        let mut value: f64 = 0.0;
        /*
         * I know this is not the correct way to get the first element in a
         * std::str::Split type, but I couldn't find any other way for now.
         */
        for element in input_split {
            value = match element.trim().parse() {
                Ok(value) => value,
                Err(_) => INFINITY
            };
            break;
        }
        if value == INFINITY {
            println!("Invalid value!\n");
            continue;
        }
        match unit {
            'F' => fahrenheit_to_celsius(value),
            'f' => fahrenheit_to_celsius(value),
            'C' => celsius_to_fahrenheit(value),
            'c' => celsius_to_fahrenheit(value),
            _ => {
                println!("{} is not a valid unit!\n", unit);
                continue;
            },
        }
        break;
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) {
    const F_TO_C: f64 = 5.0 / 9.0;
    let celsius: f64 = (fahrenheit - 32.0) * F_TO_C;
    println!("\t{} degrees Fahrenheit equals to {:.2} degrees Celsius.\n", fahrenheit, celsius);
}

fn celsius_to_fahrenheit(celsius: f64) {
    const C_TO_F: f64 = 9.0 / 5.0;
    let fahrenheit: f64 = celsius * C_TO_F + 32.0;
    println!("\t{} degrees Celsius equals to {:.2} degrees Fahrenheit.\n", celsius, fahrenheit);
}
