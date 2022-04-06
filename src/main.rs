use clap::Parser;
use std::fmt;

#[derive(Parser)]
struct Guess {
    #[clap(long = "guess")]
    word: String,
}

// This lets us pass a Guess instance to println
impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.word)
    }
}

fn main() {
    let solution = fetch_todays_solution();

    let guess = Guess::parse();

    let result = process_guess(&guess, solution);

    println!("{} --> {}", guess, result);
}

fn process_guess(guess: &Guess, solution: String) -> String {
    // Replace this with a real implementation!
    let mut response = "".to_string();

    let mut unmatched_chars = "".to_string();
    let mut result_array = "".to_string();

    for _pos in 0..guess.word.chars().count() {
        result_array.push('⬛');
    }

    for _pos in 0..guess.word.chars().count() {
        let char = guess.word.chars().nth(_pos).unwrap();

        if solution.chars().nth(_pos).unwrap() == char {
            response.push('🟩');
        } else {
            unmatched_chars.push(char);
        }
//         } else if solution.contains(char) {
//             response.push('🟨');
//         } else {
//             response.push('⬛');
//         }
    }

    response
}

fn fetch_todays_solution() -> String {
    // Hardcoded for now - maybe look at fetching a list from a server?
    "proxy".to_string()
}
