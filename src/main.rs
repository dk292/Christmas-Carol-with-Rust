fn main() {
    let song_title = "Twelve Days of Christmas";
    println!("{song_title:-^60}");

    let days: [&str;12] = [
        "a partridge in a pear tree",
        "two turtle doves,",
        "three french hens,",
        "four calling birds,",
        "FIVE GOLD RINGS,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    for day in 0..12 {
        let suffix: &str = match day + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            11 => "st",
            _ => "nd"
        };
        println!("\nOn the {}{} day of Christmas my true love gave to me", day + 1, suffix);
        for item in (0..=day).rev() {
            if day > 0 && item == 0 {
                print!("And ");
            }
            print!("{}", days[item]);
        }
        println!("\n---------------------------------")   
    }
}