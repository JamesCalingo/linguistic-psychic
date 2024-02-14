use rand::Rng;
use std::io;
use std::vec::Vec;

fn main() {
    println!("Guess the secret letter within ten guesses. Good luck!");
    let letters: [char; 26] = [
        'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k',
        'l', 'z', 'x', 'c', 'v', 'b', 'n', 'm',
    ];
    let secret_number = rand::thread_rng().gen_range(1..letters.len());
    let secret_letter = letters[secret_number].to_ascii_uppercase();
    let mut previous_guesses = Vec::new();
    let mut i = 10;
    // println!("The secret letter is {}", secret_letter);
    while i > 0 {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: char = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let uppercase  = guess.to_ascii_uppercase();
        // println!("You guessed: {}", guess);

        if uppercase == secret_letter {
            println!("That's it!");
            break;
        } else if !letters.contains(&guess.to_ascii_lowercase()) {
            println!("That's not a letter. Try again.")
        } else if previous_guesses.contains(&uppercase) {
            println!("You've guessed this already. Try something else.")
        } else {
            i -= 1;
            previous_guesses.push(uppercase);
            // This is because I don't like implying you can try after the game ends
            if i != 0 {
                println!("Nope. Try again.");
                println!("Guesses left: {}", i);
            }
        };
    }
    // I only want this line if the game ends in a loss - without the conditional it shows up when you win as well
    if i == 0 {
        println!("Game over! The answer was {}", secret_letter)
    }
}
