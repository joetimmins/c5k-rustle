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
    let mut matched = "⬛⬛⬛⬛⬛".to_string();
    for _pos in 0..guess.word.chars().count() {
        let char = guess.word.chars().nth(_pos).unwrap();
        if solution.chars().nth(_pos).unwrap() == char {
            matched.replace_range(
                    matched
                        .char_indices()
                        .nth(_pos)
                        .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                        .unwrap(),
                    "🟩",
                );
        } else if solution.contains(char) {
            matched.replace_range(
                    matched
                        .char_indices()
                        .nth(_pos)
                        .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                        .unwrap(),
                    "🟨",
                );
        }
    }
    matched
}

fn fetch_todays_solution() -> String {
    // Hardcoded for now - maybe look at fetching a list from a server?
    "proxy".to_string()
}
