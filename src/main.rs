use std::io;

fn main() {
    let upperB = 1000;
    let lowerB = 0;
    println!("Guess the number!");
    println!("I am thinking of a number between {} and {}. Please try to guess it", lowerB, upperB);
}
fn game() {
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let trimmed = guess.trim(); 
    match trimmed.parse::<u32>() {
        Ok(i) => {
        

        },
        Err(..) => println!("Oh honey that was NOT an integer..."),
    };


}