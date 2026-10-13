//! Mini-Game: Guess the Date
//!
//! Try to guess the secret date by entering a valid date in your terminal.
//! Your guess must follow the form: yyyy-mm-dd

use std::time::Duration;

fn main() {
    println!("New game! Guess the Date");

    let timeout = if let Ok(user_timeout) = std::env::var("GUESSING_TIMEOUT")
        && let Ok(user_timeout_sec) = user_timeout.parse()
    {
        Duration::from_secs(user_timeout_sec)
    } else {
        const DEFAULT_TIMEOUT: u64 = 60;

        println!("Using default timeout for guesses ({DEFAULT_TIMEOUT}s)");
        println!("Set custom timeout in seconds via 'GUESSING_TIMEOUT'");
        Duration::from_secs(DEFAULT_TIMEOUT)
    };

    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let _ = esittely::guess_date(tx, retrieve_guess, &(), timeout);
    });

    while let Ok(msg) = rx.recv_timeout(timeout) {
        println!("{msg}");
    }
}

fn retrieve_guess(
    _args: &(),
) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let stdin = std::io::stdin();
    let mut line = String::new();

    let result = stdin.read_line(&mut line);
    result
        .map(|_| line.trim().to_owned())
        .map_err(|err| Box::new(err).into())
}
