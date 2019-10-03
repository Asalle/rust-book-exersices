use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    fn guesser(secret_number: u32) {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed at reading lines");
        println!("You guessed {}", guess);
        let guess: u32 = guess.trim().parse().expect("Expected a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Guess a little more!");
                return guesser(secret_number);
            }
            Ordering::Greater => {
                println!("Guess a little less!");
                return guesser(secret_number);
            }
            Ordering::Equal => {
                println!("Guess a little... oh no you're right!");
                return;
            }
        }
    }
    return guesser(secret_number);
}
