use std::io;

fn main() {
    println!("Print the nth Fibonacci number\n");
    loop {
        println!("Please input the index of the number you want to retreive:");
        println!("Type \"exit\" to quit the program");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        println!();
        let input = input.trim();
        if input == "exit" {
            println!("Bye!");
            break;
        }
        let input: u32 = match input.parse() {
            Ok(value) => value,
            Err(value) => {
                println!("{} is an invalid value.\n", value);
                continue;
            }
        };
        let result = get_nth_fibonacci_number(input);
        println!("\tThe {}th Fibonacci number is: {}\n", input, result);
        if retry() {
            continue;
        }
        break;
    }
}

fn get_nth_fibonacci_number(number: u32) -> u64 {
    if number < 2 {
        return 0
    }
    if number < 4 {
        return 1
    }
    let mut counter: u32 = 3;
    let mut first_number: u64 = 1;
    let mut second_number: u64 = 1;
    while counter < number {
        let temp_number = first_number + second_number;
        first_number = second_number;
        second_number = temp_number;
        counter += 1;
    }
    return second_number
}

fn retry() -> bool {
    loop {
        println!("Would you want to get another numer? (Y/N)\n");
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
