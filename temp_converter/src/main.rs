use std::io;

fn main() {
    println!("============================================");
    println!("Welcome to the temperature converter cli app");
    println!("============================================");

    println!("Enter a temperature value to convert: ");
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid value for temperature");

    match input.trim().parse() {
        Ok(num) => {
            println!("Select conversion choice (type either 1 or 2 only)");
            println!("1. Celsius to Farenheit\n2. Farenheit to Celsius\n>");
            let mut temp_choice = String::new();

            io::stdin()
                .read_line(&mut temp_choice)
                .expect("Enter either 1 or 2");

            let temp_choice: i32 = temp_choice
                .trim()
                .parse()
                .expect("Invalid input received");

            if temp_choice == 1 {
                println!("{} Celsius in Farenheit is : {}", num, celsius_to_farenheit(num));
            } else if temp_choice == 2 {
                println!("{} Farenheit in Celsius is: {}", num, farenheit_to_celsius(num));
            };
        },

        Err(_) => {
            println!("Invalid input detected.\nQuitting program...");
            return;
        }
    };
}

fn celsius_to_farenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0 
}


fn farenheit_to_celsius(f: f64) -> f64 {
    5.0 / 9.0 * (f - 32.0)
}
