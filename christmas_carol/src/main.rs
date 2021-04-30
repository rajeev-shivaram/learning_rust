fn main() {
    println!("TWELVE DAYS OF CHRISTMAS!\n");

    for day in 1..13 {
        println!("{}", day_intro(day));

        for gift_day in (1..(day + 1)).rev() {
            let prefix = if gift_day == 1 && day != 1 {"and "} else { "" };
            gift(gift_day, prefix);
        }
    }
}

fn day_intro(n: u32) -> String {
    let day = match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    format!("On the {} day of Christmas my true love gave to me", day)
}

fn gift(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };

    println!("{}{}", prefix, gift_text);
}


/*
On the first day of Christmas my true love gave to me
A partridge in a pear tree

On the second day of Christmas my true love gave to me
Two turtle doves and a partridge in a pear tree

On the third day of Christmas my true love gave to me
Three French hens, two turtle doves and a partridge in a pear tree

On the fourth day of Christmas my true love gave to me
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the fifth day of Christmas my true love gave to me
Five gold rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the sixth day of Christmas my true love gave to me
Six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the seventh day of Christmas my true love gave to me
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the eighth day of Christmas my true love gave to me
Eight maids a milking, seven swans a swimming
Six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the ninth day of Christmas
Nine ladies dancing, eight maids a milking
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the tenth day of Christmas my true love gave to me
Ten lords a leaping, nine ladies dancing, eight maids a milking
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the eleventh day of Christmas my true love gave to me
Eleven pipers piping, ten lords a leaping
Nine ladies dancing, eight maids a milking
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the twelfth day of Christmas my true love gave to me
Twelve drummers drumming, eleven pipers piping
Ten lords a leaping, nine ladies dancing, eight maids a milking
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
 */