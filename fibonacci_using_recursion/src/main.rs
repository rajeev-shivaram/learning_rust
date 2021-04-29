use std::io;

fn main() {
    println!("Fibonacci Generator!");
    println!("use `q` to Quit");

    loop {
        let mut input = String::new();

        println!("Enter a value:");
        io::stdin().read_line(&mut input).expect("failed to read line");

        if input.trim().to_lowercase() == "q" {
            break;
        }

        // explicitly mention `u32` for correct parsing
        let input: u32 = input.trim().parse().expect("parse failed");

        println!("Fibonacci number is {}", fibonacci(input));

    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n -1 ) + fibonacci( n-2),
    }
}

