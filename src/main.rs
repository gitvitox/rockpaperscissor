use std::io::stdin;
use rand::{thread_rng, Rng};
use crate::robot::Roboter;
use std::process::exit;

fn main() {
    start();
}

fn start() {
    println!("Enter: Rock | Paper | Scissor");
    let input = set_input();
    check_input((&input).parse().unwrap());
    compare((&input).parse().unwrap(), robot_pick());
}

fn set_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    return input.trim().parse().unwrap();
}

fn robot_pick() -> String {
    let mut x = Roboter { choice: ["Rock", "Paper", "Scissor"], random: thread_rng().gen_range(0, 2) };
    println!("The roboter has choosen: {} \n", x.robot_choice());
    return x.robot_choice();
}

fn check_input(input: String) {
    loop {
        match &input as &str {
            "Rock" => println!("You choose: {}", &input),
            "Paper" => println!("You choose: {}", &input),
            "Scissor" => println!("You choose: {}", &input),
            _ => {
                println!("Invalid input! Try again.");
                continue;
            }
        }
        break;
    }
}

mod robot {
    pub struct Roboter<'a> {
        pub choice: [&'a str; 3],
        pub random: usize,
    }

    impl Roboter<'_> {
        pub fn robot_choice(&mut self) -> String {
            return self.choice[self.random].parse().unwrap();
        }
    }
}

fn compare(input: String, robot: String) {

    let robot = match &input as &str {

        _ if robot == input => println!("Draw!"),

        "Rock" => {
            if &robot == "Scissor" {
                println!("You win!");
            }
            if &robot == "Paper" {
                println!("You lose!");
            }
        }

        "Paper" => {
            if &robot == "Rock" {
                println!("You win!");
            }
            if &robot == "Scissor" {
                println!("You lose!");
            }
        }

        "Scissor" => {
            if &robot == "Paper" {
                println!("You win!");
            }
            if &robot == "Rock" {
                println!("You lose!");
            }
        }
        _ => println!("Error!"),
    };

    play_again();
}

fn play_again() {
    println!("Wanna play again? y / n");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    match &input.trim() as &str{
        "y" => {
            main();
        },
        "n" => {
            exit(0);
        }
        _ => println!("Invalid arguments!"),
    }
}