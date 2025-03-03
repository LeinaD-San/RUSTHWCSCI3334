fn check_number(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret: i32 = 42; 
    let mut attempts: i32 = 0;
    let mut guess: i32 = 35; 
    loop {
        attempts += 1;

        let result = check_number(guess, secret);
        
        if result == 0 {
            println!("You guessed wisely!");
            println!("The Secret Number was: {}", secret);
            break;
        } else if result == 1 {
            println!("{} too high...", guess);
            guess -= 1;
        } else {
            println!("{} too low...", guess);
            guess += 1;
        }
    }
    println!("Number of attempts: {}",attempts);
}

