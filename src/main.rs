use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "Guess the number!".blue());

    let secret_number = rand::thread_rng().gen_range(1..50);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess (1-50):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num > 50 {
                    println!("{}", "Please type a number between 1 and 50!".yellow());
                    continue;
                }
                num
            }
            Err(_) => {
                println!("{}", "Please type a number!".yellow());
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
