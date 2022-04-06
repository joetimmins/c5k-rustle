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
        write!(f, "({})", self.word.to_uppercase())
    }
}

fn main() {
    let solution = fetch_todays_solution().to_uppercase();

    let guess = Guess::parse();

    let result = process_guess(&guess, solution);

    println!("{} --> {}", guess, result);
}

fn process_guess(guess: &Guess, solution: String) -> String {
    // Replace this with a real implementation!
    // "🟩🟩🟨⬛⬛".to_string()
    let uppercase_guess = guess.word.to_uppercase();

    if uppercase_guess.eq(&solution) {
        return "🟩🟩🟩🟩🟩".to_string()
    }
    "⬛⬛⬛⬛⬛".to_string()
}

fn fetch_todays_solution() -> String {
    // Hardcoded for now - maybe look at fetching a list from a server?
    "proxy".to_string()
}
