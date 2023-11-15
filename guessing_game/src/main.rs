use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guess a number between 1 to 100!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("The secret number is : {secret_number}");

    loop {

        println!("Please input your guess!!");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

        let guess: u32 = guess.trim().parse().expect("Failed to parse input to number");

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Guessed too high!"),
            Ordering::Less => println!("Guessed too low"),
            Ordering::Equal => {
                println!("You win!");
                break;
        }
    }

    }

}
