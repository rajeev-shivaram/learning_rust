use std::io;

fn main() {
    println!("Fibonacci sequence generator\n enter 'q' to exit");

    loop {
        let mut input = String::new();
        println!("Enter the fib no:");
        io::stdin().read_line(&mut input).expect("failed to read");

        if input.trim().to_lowercase() == "q" {
            break;
        }

        let input: usize = input.trim().parse().expect("failed to parse");
        let mut fib_ser: Vec<u64> = vec![0, 1];

        while input >= fib_ser.len() {
            let next: u64 = fib_ser[fib_ser.len() - 1] + fib_ser[fib_ser.len() - 2];
            fib_ser.push(next);
        }

        println!("Fibonacci number is {}", fib_ser[fib_ser.len() - 1])
    }
}
