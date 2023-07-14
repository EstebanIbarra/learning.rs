fn main() {
    let string = String::from("String type");
    println!("This is a String type: {}", string);
    // first_word works on slices of `String`s
    let word = first_word(&string[..]);
    println!("    First word of String slice: {}\n", word);
    let string_literal = "string literal";
    println!("This is a string literal: {}", string_literal);
    // first_word works on slices of string literals
    let word = first_word(&string_literal[..]);
    println!("    First word of string literal slice: {}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(string_literal);
    println!("    First word of string literal: {}\n", word);
    println!("You can slice arrays too!\n");
    let array = [1, 2, 3, 4, 5];
    println!("This is an array: {:?}", array);
    let array_slice = &array[1..3];
    println!("    This is a slice of the array: {:?}", array_slice);
}

/*
 * In Rust you can slice a string using the following syntax:
 * &string[0..4]
 * &string[..4]
 * &string[0..]
 * &string[..]
 * The first number is the starting index and the second number is the ending index.
 * If you don't specify the starting index, it will start from 0.
 * If you don't specify the ending index, it will end at the last index.
 * The ending index is exclusive, so the above examples will return "Hell".
 */
fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // The following two lines are equivalent
            return &string[..i];
            // return &string[0..i];
        }
    }
    // The following two lines are equivalent
    return &string[..];
    // return &string[0..];
}
