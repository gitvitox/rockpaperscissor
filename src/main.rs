use std::io::stdin;
use rand::{thread_rng, Rng};
use crate::robot::Roboter;

fn main() {
    start();
}

fn start() {
    println!("Enter: Rock | Paper | Scissor");
    let mut x = Roboter {choice: ["Rock", "Paper", "Scissor"], random: thread_rng().gen_range(0, 2)};
    check_input();
    println!("The roboter has choosen: {}", x.robot_choice());
}

fn set_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    return input.trim().parse().unwrap();
}

fn check_input() {
    loop {
        match &set_input() as &str {
            "Rock" => println!("You choose rock!"),
            "Paper" => println!("You choose paper!"),
            "Scissor" => println!("You choose scissor!"),
            _ => {
                println!("Invalid input! Try again.");
                continue;
            },
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