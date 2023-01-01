use rand::Rng;
use std::io;

fn main() {
    let lower_bound = determin_lower_bound();
    let upper_bound = determin_upper_bound();

    println!(
        "Ok, let's guess a number between {} and {}.",
        lower_bound, upper_bound
    );

    let secret_num: i32 = number_range(lower_bound, upper_bound);

    println!("{}", secret_num);

    loop {
        let input: i32 = player_guess();

        if input == secret_num {
            println!("You guessed correctly! The number was {}.", secret_num);
            break;
        } else if input > secret_num {
            println!("Too high, guess again.");
        } else {
            println!("Too low, guess again.");
        }
    }

    fn number_range(x: i32, y: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(x..=y)
    }

    fn determin_lower_bound() -> i32 {
        println!("Select a range of numbers to guess from. select the lower bound");
        let mut lower = String::new();
        io::stdin()
            .read_line(&mut lower)
            .expect("Error selecting lower bound");
        lower.trim().parse().unwrap()
    }

    fn determin_upper_bound() -> i32 {
        println!("Now please select an upper bound.");
        let mut upper = String::new();
        io::stdin()
            .read_line(&mut upper)
            .expect("Error occured at upper bound selection.");
        upper.trim().parse().unwrap()
    }

    fn player_guess() -> i32 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("An error occured");
        input.trim().parse().unwrap()
    }
}
