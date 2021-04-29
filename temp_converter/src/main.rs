use std::io;

fn main() {
    println!(" Temperature Convertor");
    println!("type \"q\" to quit\ntype \"f\" to convert from Fahrenheit to Celsius\ntype \"c\" to \
    convert from Celsius to Fahrenheit.");

    loop{
        println!("Enter type of operation:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read");

        if input.trim() == "q" {
            break;
        } else if input.trim().to_lowercase() == "f" {
            // call fc fn
            let value = capture_value();
            println!("{}°F -> {}°C", value, fahrenheit_to_celsius(value));
            continue
        } else if input.trim().to_lowercase() == "c" {
            let value = capture_value();
            println!("{}°C -> {}°F", value, celsius_to_fahrenheit(value));
            continue
        } else {
            println!("select correct input");
            continue;
        }

    }
}

fn capture_value() -> f64 {
    println!("Enter the value:");

    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("failed to read value");

    let value: f64 = value.trim().parse().expect("failed to parse value");

    value
}

fn fahrenheit_to_celsius(fah: f64) -> f64 {
    ( fah - 32 as f64 ) * (f64::from(5)/ f64::from(9) )
}

fn celsius_to_fahrenheit(cel: f64) -> f64 {
    ( cel * ( 9.0/5.0) ) + 32.0
}
