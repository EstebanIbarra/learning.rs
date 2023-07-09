fn main() {
    let x = 5;
    println!("The original value of x is: {}", x);
    let x = x + 1;
    println!("The first shadow value of x is: {}", x);
    let x = x * 2;
    println!("The second shadow value of x is: {}", x);
    /* The main difference between using mut and shadowing is that shadowing
     * allows us to change the data type that is being used.
     * In the following examples, the shadowing of the spaces variable works
     * because the .len() method returns a usize data type, so it's creating
     * a new variable with the usize type, therefore no compiler error.
     * The example using mut tries to reassign a usize value to a str type
     * variable, therefore throwing a mismatched type error at compiling.
     */
    let spaces = "     ";
    let spaces = spaces.len();
    /* let mut spaces = "     ";
    spaces = spaces.len(); */
    println!("The current value of spaces is: {}", spaces);
}
