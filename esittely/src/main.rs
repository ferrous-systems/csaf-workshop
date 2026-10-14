//! Mini-Game: Guess the Date
//!
//! Try to guess the secret date by entering a valid date in your terminal.
//! Your guess must follow the form: yyyy-mm-dd

use std::process;

fn main() {
    // TODO: Change this to `esittely::guess_date_v2` to fix the vulnerability
    match esittely::guess_date_v1() {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("Error while playing: {err}");
            process::exit(1);
        }
    }
}
