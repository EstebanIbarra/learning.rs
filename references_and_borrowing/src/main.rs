fn main() {
    let string = String::from("Hello");
    /*
     * The ampersands are references, and they allow you to refer to some value
     * without taking ownership of it.
     */
    let length = calculate_length(&string);
    /*
     * The &string syntax lets us create a reference that refers to the value of
     * string but does not own it. Because it does not own it, the value it
     * points to will not be dropped when the reference goes out of scope.
     */
    println!("The length of '{}' is {}.\n", string, length);
    let mut string = string;
    change(&mut string);
    println!("Prints borrowed variable to change function from main function: {}\n", string);
    will_fail();
    will_also_fail();
}

fn calculate_length(string: &String) -> usize {
    /*
     * When functions have references as parameters instead of the actual values,
     * we won’t need to return the values in order to give back ownership, because
     * we never had ownership.
     */
    return string.len()
}

// fn change(string: &String) {
    /*
     * This function takes a reference to a String. The scope within which the
     * variable string is valid is the same as any function parameter’s scope,
     * but we don’t drop what the reference points to when it goes out of scope
     * because we don’t have ownership. References are immutable by default, so
     * we’re not allowed to modify the value string points to.
     */
    // string.push_str(", world!");
// }
// error[E0596]: cannot borrow immutable borrowed content `*string` as mutable

fn change(string: &mut String) {
    /*
     * We can fix this error by changing the parameter to be a mutable reference
     * by adding mut in two places.
     *
     * But mutable references have one big restriction: you can have only one
     * mutable reference to a particular piece of data in a particular scope.
     */
    string.push_str(", world!");
}

fn will_fail() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    /*
     * The scope of r1 ends after the println! statement, which means we’re ready
     * to make a new reference with r2. But we can’t, because the immutable
     * borrow r1 is still in scope until the end of the outer scope. This code
     * will fail:
     */
    // let r2 = &mut s;
    /*
     * error[E0499]: cannot borrow `s` as mutable more than once at a time
     *   --> main.rs:52:17
     *    |
     * 48 |     let r1 = &mut s;
     *    |              ------ first mutable borrow occurs here
     * 49 |     println!("{}", r1);
     * 50 |     let r2 = &mut s;
     *    |                 ^^ second mutable borrow occurs here
     * 51 | }
     *    | - first borrow ends here
     */
    println!("{}", r1);
    /*
     * The benefit of having this restriction is that Rust can prevent data races
     * at compile time.
     * A data race is similar to a race condition and happens when these three
     * behaviors occur:
     * - Two or more pointers access the same data at the same time.
     * - At least one of the pointers is being used to write to the data.
     * - There’s no mechanism being used to synchronize access to the data.
     */
}

fn will_also_fail() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    /*
     * These references have different scopes, so the code is allowed. These
     * borrows also do not conflict with the mutable reference in Listing 4-10
     * because we aren’t trying to change the value at all.
     */
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    /*
     * The scopes of the immutable references r1 and r2 end after the println!
     * where they are last used, which is before the mutable reference r3 is
     * created. These scopes don’t overlap, so this code is allowed.
     */
    println!("{}", r3);
}

// fn dangle() -> &String {
    /*
     * This function’s return type contains a &, meaning it returns a reference.
     * But what is it a reference to? We don’t have anything, because the
     * String we created in the function will be deallocated when the function
     * ends. There won’t be anything for the reference to refer to.
     */
    // let s = String::from("hello");
    // return &s
    /*
     * If Rust allowed this code to work, s would be deallocated when it went out
     * of scope at the end of the function. Meanwhile, we would have a dangling
     * reference as the return value. That’s no good! Rust prevents this from
     * happening by refusing to compile.
     */
// }
