fn main() {
    // SCALAR TYPES
    // Integer types
    let int: i8 = -0xf;
    println!("This is an i8 data type: {}", int);
    let int: u8 = b'A';
    println!("This is an u8 data type: {}", int);
    let int: i16 = -129;
    println!("This is an i16 data type: {}", int);
    let int: u16 = 0o77;
    println!("This is an u16 data type: {}", int);
    // The default int type in Rust is i32
    let int: i32 = -0b1111_0000;
    println!("This is an i32 data type: {}", int);
    let int: u32 = 0b1111_1111;
    println!("This is an u32 data type: {}", int);
    let int: i64 = -1234;
    println!("This is an i64 data type: {}", int);
    let int: u64 = 1234;
    println!("This is an u64 data type: {}", int);
    // Both isize and usize are dependant on the processor architecture
    let int: isize = -2345;
    println!("This is an isize data type: {}", int);
    let int: usize = 2345;
    println!("This is an usize data type: {}", int);

    // Floating-Point types
    let float = 2.5;  // f64
    println!("This is a f64 data type: {}", float);
    let float: f32 = 3.65;  // f32
    println!("This is a f32 data type: {}", float);

    // Numeric Operations
    // addition
    let sum = 5 + 10;
    // substraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    println!("addition: {}\nsubstraction: {}\nmultiplication: {}\ndivision: {}\nremainder: {}", sum, difference, product, quotient, remainder);

    // Boolean Type
    let t = true;
    let f: bool = false;
    if t {
        println!("This printed because t is: {}", t);
    }
    if !f {
        println!("This printed because f is: {}", f);
    }

    // Character Type
    /*
     * In Rust as in C/C++ the string type is notated with double quotes ("")
     * meanwhile the single character type is notated with simple quotes ('')
     */
    let c = 'z';
    let z = '\u{10ffff}';
    let pinching_fingers = '\u{1f90c}';
    println!("first character: {}\nsecond character: {}\npinching fingers emoji: {}", c, z, pinching_fingers);
}
