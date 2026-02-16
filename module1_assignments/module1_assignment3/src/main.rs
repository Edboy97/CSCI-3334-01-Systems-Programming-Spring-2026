fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    }
    else if guess > secret {
        1
    }
    else {
        -1
    }
}

fn main() {
    let mut secret_number = 25;
    let simulated_guesses = [4, 15, 29, 25];
    let mut guess_count = 0;
    let mut current_guess: i32;
    for &guess in simulated_guesses.iter(){
        current_guess = guess;
        guess_count += 1;
        let result = check_guess(current_guess, secret_number);
        if current_guess == secret_number{
            println!("Guess {} You are correct ", current_guess);
            break;
        }
        else if current_guess > secret_number{
            println!("Guess {} Too high buddy ", current_guess);
        }
        else {
            println!("Guess {} Too low buddy ", current_guess);
        }
    }
    println!("It took you {} of guesses", guess_count);
}
