use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Input your number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","Correct Guess , You Win !".green());
                break;
            }
            Ordering::Greater => println!("{}","Too Big !".red()),
            Ordering::Less => println!("{}","Smol guess".red()),
        }
    }
}
