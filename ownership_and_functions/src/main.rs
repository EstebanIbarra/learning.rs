fn main() {
    let string: String = String::from("hello");  // variable string comes into scope
    takes_ownership(string);                     // string's value moves into the function...
                                                 // ... and so is no longer valid here
    // string.push_str(" world!");               // Comp: `string` value borrowed here after move
    let x = 5;                                   // x comes into scope
    let y = String::from("test");                // y comes into scope
    let string2 = takes_and_gives_back(y);       // y is moved into the function
    println!("This is a burrow test:\n\tVariable in scope: {}\n\tVariable with ownership given back: {}\n", x, string2);
    let (string3, length) = calculate_length(string2);
    println!("The length of '{}' is {}.", string3, length);
    drop(string3);                               // you can explicitly drop a variable
                                                 // but it's not necessary
                                                 // because Rust will do it automatically
                                                 // when the variable goes out of scope
    // println!("{}", string3);                  // Comp: value borrowed here after move
}

fn takes_ownership(mut string: String) {
    string.push_str(" world from takes_ownership function!");
    println!("{}", string);
}
 fn takes_and_gives_back(mut string: String) -> String {
     string.push_str(" <this should give back the ownership to the calling function>");
     return string
 }

fn calculate_length(string: String) -> (String, usize) {
    let length = string.len();
    (string, length)
}
