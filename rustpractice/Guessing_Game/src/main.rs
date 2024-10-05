fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 42;
    let mut guess_count = 0;
    let mut guesses = [36, 50, 25, 40, 42]; // Simulated user inputs

    loop {
        let guess = guesses[guess_count];
        guess_count += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The number was {}", secret);
            break;
        } else if result == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }

        if guess_count == guesses.len() {
            println!("Out of guesses. The number was {}", secret);
            break;
        }
    }

    println!("It took {} guesses", guess_count);
}