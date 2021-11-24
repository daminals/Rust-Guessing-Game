use std::io;
use rand::Rng;

fn main() {
    let upperB: i32 = 1000;
    let lowerB: i32 = 0;
    let mut rng = rand::thread_rng();
    println!("Guess the number!");
    println!("I am thinking of a number between {} and {}. Please try to guess it", lowerB, upperB);
    let correct_num = rng.gen_range(lowerB..upperB);
    game(correct_num, 0);
}
fn game(correct_num: i32, mut guessN: i32) -> bool {
    guessN += 1;
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    let guess_int = guess.trim().parse::<i32>().unwrap();
    if  guess_int == correct_num {
        println!("Yes! The correct guess was {}! You have won this game in only {} guesses!", guess_int, guessN);
        return true;
    } else {
        if guess_int > correct_num {
            println!("No you silly goose, {} is GREATER than my number lol",guess_int);
        }
        else {
            println!("No you silly goose, {} is LOWER than my number lol",guess_int);
        }
        game(correct_num, guessN);
    }
    return false;
}