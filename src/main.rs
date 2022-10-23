use std::io;

fn main() {
    println!("Guess the number between 1 and 100!:\n");

    println!("Please input your guess here.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}
