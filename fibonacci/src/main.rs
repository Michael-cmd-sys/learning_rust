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
    
    println!("You just typed : {}", number);
    let number: i64 = number
            .trim()
            .parse()
            .expect("Invalid number received");
    println!("Value after parsing is: {}", number);
    println!("The {}th Fibonacci number is: {}", number, fib(number));
}

// TODO: Find out if there's a way to include optional return like in js
fn fib(n: i64) -> i64 {
    if n < 0 {
        return 0;
    }

    return match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    };
}
