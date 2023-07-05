fn main() {
    /* The following variable declaration causes a compiler error
     * because variables without a mut constrain cannot be reassigned
     */
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
