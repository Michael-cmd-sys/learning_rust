use std::io;

fn main() {
    println!("===============================");
    println!("Welcome to the classic Fib game");
    println!("===============================");
    let mut number = String::new();
    
    println!("Enter the nth number of the fibonacci sequence\n that you want to find: ");
    io::stdin()
        .read_line(&mut number)
        .expect("Enter a valid numver");
    
    let number: i64 = number
            .trim()
            .parse()
            .expect("Invalid number received");

    println!("Value after parsing is: {}", number);
    println!("The {}th Fibonacci number is: {:#?}", number, fib(number));
}

// TODO: Find out if there's a way to include optional return like in js
fn fib(n: i64) -> Option<i64> {
    if n < 0 {
        return None;
    }

    match n {
        0 => Some(0),
        1 => Some(1),
        _ => {
            match (fib(n-1), fib(n-2)) {
                (Some(a), Some(b)) => Some(a+b),
                _ => None,
            }
        }
    };
}
