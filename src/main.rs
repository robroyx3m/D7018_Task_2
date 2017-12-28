extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0; 

    loop{

        tries += 1;

        println!("Please input your guess.");

        let guess = match input(){
            Ok(num) => num,
            Err(error) => {
                println!("{}",error);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                println!("It took {} tries to finish!", tries);
                break;
            }
        }

        println!("Current nr. of tries: {} \n", tries);
    }
}

fn input() -> Result<u32, String> {

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
            .expect ("Failed to read line");
    
    match guess.trim().parse() {
            Ok(num) => Ok(num),
            Err(error) => Err(format!("{}{}","in parsing to u32, ", error)),
    }
}