use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess The number !");

    
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Secret Number: {}", secret_number);

        println!("Please input your address");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

    // guess.cmp(&secret_number);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Equal => println!("{}", "Too Big".red()),
            Ordering::Greater => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}