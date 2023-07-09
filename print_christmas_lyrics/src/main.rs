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
    println!("Twelve Days of Christmas\n\n");
    for i in 0..12 {
        let lyrics: (&str, &str) = LYRICS[i];
        let ordinal_number: &str = lyrics.0;
        println!("On the {} day of Christmas, my true love sent to me", ordinal_number);
        let mut counter = i + 1;
        while counter > 0 {
            let lyric: &str = LYRICS[counter - 1].1;
            println!("{}", lyric);
            counter -= 1;
        }
        println!();
    }
}
