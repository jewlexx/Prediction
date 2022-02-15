use rand::Rng;
use spinners_rs::{Spinner, Spinners};
use std::io::stdin;

const BALL_OPTIONS: [&str; 20] = [
    "It is certain",
    "It is decidedly so",
    "Without a doubt",
    "Yes - definitely",
    "You may rely on it",
    "As I see it, yes",
    "Most likely",
    "Outlook good",
    "Yes",
    "Signs point to yes",
    "Reply hazy, try again",
    "Ask again later",
    "Better not tell you now",
    "Cannot predict now",
    "Concentrate and ask again",
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "Outlook not so good",
    "Very doubtful",
];

fn rand_in_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

fn main() {
    print!("What would you like to know? ");
    stdin().read_line(&mut String::new()).unwrap();

    let sp = Spinner::new(Spinners::Dots9, "Divining your answer...".into());

    let answer_i = rand_in_range(0, (BALL_OPTIONS.len() - 1).try_into().unwrap());
    let answer = BALL_OPTIONS[answer_i as usize];

    sp.stop_with_message(format!("Magic 8 ball says: {}", answer));
}
