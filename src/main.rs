fn main() {
    let th_values = [
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
        "twelveth",
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "fiiiiiive goooolden riiiiiings",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me", th_values[day]);

        for n in (0..day+1).rev() {
            if day != 0 && n == 0 {
                println!("and {}", gifts[n]);
            } else {
                println!("{}", gifts[n]);
            }
        }
        println!("");
    }
}
