use std::io;
use std::io::{Write};
use rand::Rng;

fn main() {
    let mut input = String::new();

    loop {
        play(&mut input);

        print("Would you like to play again? (y?): ");
        read_input(&mut input);
        if input.ne("y") {
            println!("Thanks for playing!");
            break;
        }
    }
}

fn play (input: &mut String) {
    let mut rng = rand::thread_rng();
    let min = 1;
    let max = 10;
    let number = rng.gen_range(min..=max);

    let mut lives: i8 = 5;

    println!("---------------------------------------");
    println!("I picked a number between {} and {}.", min, max);
    println!("Try to guess it.");

    while lives > 0 {
        println!("You have {} lives left.", lives);
        println!("---------------------------------------");
        print("Guess: ");
        read_input(input);

        let guessed_number: u8;
        match input.parse() {
            Ok(number) => guessed_number = number,
            _ => {
                println!("Could not understand your guess. Please try again.");
                continue;
            }
        }

        if guessed_number < min || guessed_number > max {
            println!("{} is not between {} and {}", guessed_number, min, max);
            continue;
        }

        if guessed_number == number {
            println!("You Win! 🎉");
            return;
        } else {
            lives -= 1;

            if lives > 0 {
                if number > guessed_number {
                    println!("Nope. Higher.");
                } else {
                    println!("Nope. Lower.");
                }
            }
        }
    }
    println!("Sorry. You lose! 😢");
}

fn read_input(string: &mut String) {
    string.clear();
    io::stdin().read_line(string).unwrap();
    string.pop(); // Trailing \n
}

fn print(str: &str) {
    print!("{}", str);
    io::stdout().flush().unwrap();
}
