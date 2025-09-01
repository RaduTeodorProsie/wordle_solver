mod solver;

use crate::solver::{Word, get_words};
use solver::{Info, best_guess};
use std::collections::HashSet;

fn parse_feedback(s: &str) -> Option<[Info; 5]> {
    let chars: Vec<char> = s.trim().chars().filter(|c| !c.is_whitespace()).collect();

    if chars.len() != 5 {
        return None;
    }

    let mut arr = [Info::Grey; 5];
    for i in 0..5 {
        arr[i] = match chars[i] {
            'G' | 'g' => Info::Green,
            'Y' | 'y' => Info::Yellow,
            'B' | 'b' => Info::Grey,
            _ => return None,
        };
    }
    Some(arr)
}

fn is_valid_word(s: &str, valid_words: &HashSet<Word>) -> bool {
    if s.len() != 5 {
        return false;
    }
    let word: Word = match s.as_bytes().try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    valid_words.contains(&word)
}
fn main() {
    let valid_words = get_words().iter().copied().collect::<HashSet<_>>();

    println!(
        "This is a worlde solver.\nThe feedback can be given in either uppercase or lowercase:\nG for green, Y for yellow and B for grey\n\n"
    );

    let mut guesses: Vec<[u8; 5]> = vec![];
    let mut feedbacks: Vec<[Info; 5]> = vec![];

    while feedbacks.last() != Some(&[Info::Green; 5]) {
        let next = best_guess(&guesses, &feedbacks);
        println!("Best guess is: {}", String::from_utf8_lossy(&next));

        loop {
            println!("What's your guess?");
            let mut guess = String::new();
            std::io::stdin().read_line(&mut guess).unwrap();
            let guess = guess.trim();
            if !is_valid_word(&guess, &valid_words) {
                println!("Invalid word found");
                continue;
            }

            println!("\nWhat's the feedback for {}?", guess);
            let mut feedback_string = String::new();
            std::io::stdin().read_line(&mut feedback_string).unwrap();
            if let Some(feedback) = parse_feedback(&feedback_string) {
                feedbacks.push(feedback);
                guesses.push(guess.as_bytes().try_into().unwrap());
                break;
            } else {
                println!("Invalid feedback");
            }
        }
    }

    println!("The word was guessed! Woohoo!");
}
