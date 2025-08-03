use std::{cmp::Ordering, io};

use rand::Rng;

fn main(){

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is: {secret_number}");

    println!("please input your guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("you guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("to small!"),
        Ordering::Greater => println!("to large!"),
        Ordering::Equal => println!("You win!"),
    }
}
