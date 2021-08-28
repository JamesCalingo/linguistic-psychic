use std::io;

fn main() {
    println!("Guess the letter!");
    println!("Input guess now.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    println!("You guessed: {}", guess)
}
