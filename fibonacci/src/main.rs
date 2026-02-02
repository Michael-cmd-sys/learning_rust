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

    match fib(number) {
        Ok(value) => println!("The {}th Fibonacci number is: {:#?}", number, value),
        Err(err) => eprintln!("Error: {}", err),

    }
}

fn fib(n: i64) -> Result<i64, String> {
    if n < 0 {
        return Err("Fibonnaci is not implemented for negative numbers".to_string());
    }

    match n {
        0 => Ok(0),
        1 => Ok(1),
        _ => {
            let a = fib(n-1)?;
            let b = fib(n-2)?;

            Ok(a+b)
        }
    }
}
