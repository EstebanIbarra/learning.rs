const LYRICS: [(&str, &str); 12] = [
    ("first", "A partridge in a pear tree"),
    ("second", "Two turtle doves, and"),
    ("third", "Three french hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Five golden rings"),
    ("sixth", "Six geese a-laying"),
    ("seventh", "Seven swans a-swimming"),
    ("eighth", "Eight maids a-milking"),
    ("ninth", "Nine ladies dancing"),
    ("tenth", "Ten lords a-leaping"),
    ("eleventh", "Eleven pipers piping"),
    ("twelfth", "Twelve drummers drumming"),
];
fn main() {
    println!("Twelve Days of Christmas\n");
    for lyrics in LYRICS {
        println!("On the {} day of Christmas, my true love sent to me", lyrics.0);
        let mut i: usize = LYRICS.iter().position(|&e| e == lyrics).unwrap() + 1;
        while i > 0 {
            let lyric: &str = LYRICS[i - 1].1;
            println!("{}", lyric);
            i -= 1;
        }
        println!();
    }
}
