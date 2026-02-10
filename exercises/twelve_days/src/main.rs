fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "nineth",
        "tenth", "eleventh", "twelfth",
    ];
    const GIFTS: [&str; 12] = [
        "Partridge in a Pear Tree",
        "Turtle Doved",
        "French Hens",
        "Calling Birds",
        "Golden rings",
        "Geese a Laying",
        "Swans a Swimming",
        "Maids a Milking",
        "Ladies Dancing",
        "Lords a Leaping",
        "Pipers Piping",
        "Drummers Drumming",
    ];

    for day in 1..=12 {
        let current_day = DAYS[day - 1];
        println!("On the {current_day} day of Christmas");
        println!("my true love sent to me:");

        for gift in (1..=day).rev() {
            let current_gift = GIFTS[gift - 1];
            if day == 1 && gift == 1 {
                println!("A {current_gift}");
            } else if gift == 1 {
                println!("And a {current_gift}");
            } else {
                println!("{gift} {current_gift}");
            }
        }
    }
}
