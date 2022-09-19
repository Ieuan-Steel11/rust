use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut score: i32 = 0;
    let quit: bool = false;
    // init score var and quit condition

    while quit != true {

        let target: i32 = rng.gen_range(0..11);

        println!("Guess the value of the number?");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        // reads entered line and handles errors

        let guess: i32 = guess.trim().parse().unwrap();
        // convert user guess from string to i32

        if guess == target { // checks whether guess is correct
            println!("Correct!");
            score += 1;
        } else {
            println!("Wrong!");
        }

        println!("Your current score is {score}");
        println!("Press Ctrl + C to quit \n")
    }
}
