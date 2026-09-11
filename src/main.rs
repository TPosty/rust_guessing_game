use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed {guess}");
    // Making this change to test out
    // Making another change in this file

    // This is a change from ONLY the second branch...
}
