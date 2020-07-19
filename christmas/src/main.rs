fn main() {
    let days = [
        "first", 
        "second", 
        "third", 
        "fourth", 
        "fifth", 
        "sixth", 
        "seventh", 
        "eighth", 
        "ninth", 
        "tenth", 
        "eleventh", 
        "twelfth",
    ];
    let day_lyrics = [
        "A partridge in a pear tree", 
        "Two turtle doves", 
        "Three French hens", 
        "Four calling birds", 
        "Five gold rings", 
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "12 drummers drumming",
    ];

    for (index, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas", day);
        println!("My true love gave to me");
        for lyrics in (0..index + 1).rev() {
            println!("{}", day_lyrics[lyrics]);
        }
    }
}
