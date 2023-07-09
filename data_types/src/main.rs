fn main() {
    println!("SCALAR TYPES");
    println!("Integer Data Types:");
    let int8: i8 = -0xf;
    let u_int8: u8 = b'A';
    let int16: i16 = -129;
    let u_int16: u16 = 0o77;
    // The default int type in Rust is i32
    let int32: i32 = -0b1111_0000;
    let u_int32: u32 = 0b1111_1111;
    let int64: i64 = -1234;
    let u_int64: u64 = 1234;
    // Both isize and usize are dependant on the processor architecture
    let int_arch: isize = -2345;
    let u_int_arch: usize = 2345;
    println!("\ti8: {}\n\tu8: {}\n\ti16: {}\n\tu16: {}\n\ti32(default): {}\n\tu32: {}\n\ti64: {}\n\tu64: {}\n\tisize: {}\n\tusize: {}\n", int8, u_int8, int16, u_int16, int32, u_int32, int64, u_int64, int_arch, u_int_arch);

    println!("Floating-Point Data Types:");
    let float64 = 2.5;  // f64
    let float32: f32 = 3.65;  // f32
    println!("\tf32: {}\n\tf64(default): {}\n", float32, float64);

    println!("Numeric Operations:\n    You can assign a variable the result of a numeric operation.");
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
    println!("\taddition: {}\n\tsubstraction: {}\n\tmultiplication: {}\n\tdivision: {}\n\tremainder: {}\n", sum, difference, product, quotient, remainder);

    println!("Boolean Data Type:");
    let t = true;
    let f: bool = false;
    println!("\tTRUE: {}\n\tFALSE: {}\n", t, f);

    println!("Character Data Type:\n    You can assign a variable any unicode scalar value ranging from U+0000 to U+D7FF and U+E000 to U+FFFF.");
    /*
     * In Rust as in C/C++ the string type is notated with double quotes ("")
     * meanwhile the single character type is notated with simple quotes ('')
     */
    let c = 'z';
    let z = '\u{10ffff}';
    let pinching_fingers = '\u{1f90c}';
    println!("\tfirst character: {}\n\tsecond character: {}\n\tpinching fingers emoji: {}\n\n", c, z, pinching_fingers);

    println!("COMPOUND TYPES");
    println!("Tuple Data Type:");
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("    This is a tuple:\n\tlet tuple = ({}, {}, {});\n", tuple.0, tuple.1, tuple.2);
    // Type annotations are optional when types are simple scalar types
    let tuple = (500, 6.4, 1);
    println!("    You can access a tuple value by it's index:\n\ttuple.0: {}\n\ttuple.1: {}\n\ttuple.2: {}\n", tuple.0, tuple.1, tuple.2);
    let (x, y, z) = tuple;
    println!("    A tuple can be destructured like this:\n\tlet (x, y, z) = tuple;\n");
    println!("    The values of the destructured tuple are:\n\tx: {}\n\ty: {}\n\tz: {}\n", x, y, z);

    println!("Array Data Type:");
    // Type annotations are optional when type is simple scalar types
    let array: [i8; 5] = [1, 2, 3, 4, 5];
    println!("    This is an array:\n\tlet array = [{}, {}, {}, {}, {}];\n", array[0], array[1], array[2], array[3], array[4]);
    println!("    You can predefine the length and the data type of an array:\n\tlet array: [i8; 5] = [{}, {}, {}, {}, {}];\n", array[0], array[1], array[2], array[3], array[4]);
    let array: [&str; 5] = ["string"; 5];
    println!("    You can initialize an array:\n\tlet array: [&str; 5] = [\"string\"; 5];\n      which will create the array:\n\t[\"{}\", \"{}\", \"{}\", \"{}\", \"{}\"]\n", array[0], array[1], array[2], array[3], array[4]);
    let array: [i8; 5] = [1, 2, 3, 4, 5];
    println!("    You can access an array value by it's index:\n\tarray[0]: {}\n\tarray[1]: {}\n\t...\n", array[0], array[1]);
    let [a, b, c, d, e] = array;
    println!("    An array can be destructured like this:\n\tlet [a, b, c, d, e] = array;\n");
    println!("    The values of the destructured array are:\n\ta: {}\n\tb: {}\n\tc: {}\n\td: {}\n\te: {}\n", a, b, c, d, e);

    println!("Functions:");
    println!("    Functions must be declared with the following syntax:\n\tfn <function name>(<parameters>) {{\n\t\t<code to be executed>\n\t}}\n");
    println!("      Example:\n\tfn example_function(param: i32) {{\n\t\tprintln!(\"The value of param is: {{}}\", param);\n\t}}\n");
    println!("    Functions can be called by typing their name and passing the parameters it requires.\n      Example:\n\texample_function(1234);");
    print!("\t");
    example_function(1234);
    println!("    If a function has a return value, the syntax should be:\n\tfn <function name>(<parameters>) -> <return type> {{\n\t\t<code to be executed>\n\t}}\n");
    println!("      Example:\n\tfn example_function2(param: i32) -> i32 {{\n\t\tparam + 1\n\t}}\n");
    println!("      NOTE: By default the last line of a function body must be the return value,\n\tit musn't have a semicolon (;)\n\tthe return keyword is not needed unless returned early.\n");
    println!("    If a function has a return value, we can assign this value to another variable:\n\tlet variable = example_function2(1234);\n");
    let variable: i32 = example_function2(1234);
    print!("\t");
    example_function(variable);
}

fn example_function(param: i32) {
    println!("The value of param is: {}\n", param);
}

fn example_function2(param: i32) -> i32 {
    param + 1
}
