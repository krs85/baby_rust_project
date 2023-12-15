use std::io;

use baby_rust_project::{other_stuff, some_stuff};

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    other_stuff::idk();
    some_stuff::goodbye();
}
