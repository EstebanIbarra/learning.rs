use std::io;

enum Unit {
    Fahrenheit,
    Celsius,
    Undefined,
}

fn main() {
    println!("Fahrenheit-Celsius Converter\n");
    loop {
        println!("Type \"exit\" to quit the program\n");
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
        let (value, unit) = match parse_value_and_unit(input) {
            Ok((value, unit)) => (value, unit),
            Err(_) => continue,
        };
        match unit {
            Unit::Fahrenheit => fahrenheit_to_celsius(value),
            Unit::Celsius => celsius_to_fahrenheit(value),
            _ => continue,
        }
        if retry() {
            continue;
        }
        println!("Bye!");
        break;
    }
}

fn parse_value_and_unit(input: &str) -> Result<(f64, Unit),()> {
    let bytes = input.as_bytes();
    let mut value: f64 = 0.0;
    let mut unit: Unit = Unit::Undefined;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'F' || item == b'f' || item == b'C' || item == b'c' {
            value = match input[..i].trim().parse() {
                Ok(parsed_value) => parsed_value,
                Err(_) => {
                    println!("{} is not a valid value!\n", input[..i].trim());
                    break;
                },
            };
            if item == b'F' || item == b'f' {
                unit = Unit::Fahrenheit;
            } else {
                unit = Unit::Celsius;
            }
        }
    }
    match unit {
        Unit::Undefined => Err(()),
        _ => Ok((value, unit)),
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

fn retry() -> bool {
    loop {
        println!("Would you want to convert another temperature? (Y/N)\n");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input: char = input.trim().chars().last().unwrap();
        let retry = match input {
            'Y' => true,
            'y' => true,
            'N' => false,
            'n' => false,
            _ => {
                println!("Invalid option");
                continue;
            }
        };
        println!();
        return retry
    }
}
