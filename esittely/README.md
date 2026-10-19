# `esittely` - Guess the Date

This crate implements the "Guess the Date" game.
It is intended to be played in a terminal.

## Play in Terminal

To play the game in your terminal, ensure you are located in the crate directory and execute:

```sh
cargo run
```

This will start the game and print something like:

```
New game! Guess the Date
Using default timeout for guesses (60s)
Set custom timeout in seconds via 'GUESSING_TIMEOUT'
Your guess must follow the form: yyyy-mm-dd
e.g. 2026-10-13

Years are in the range: 1..=9999
Years before 1000, no padding is needed
e.g. 100-10-13
---------------------------------
Enter guess:
```

Now it is your turn to guess the secret date.
Each guess will tell you if the secret date is *earlier* or *later* than your guess.

## Crate Structure

The game logic is handled in `esittely::guess_date()`, which takes in a function to retrieve a new guess from the terminal.
Two versions for retrieving user input are implemented as `esittely::guess_date_v1()` and `esittely::guess_date_v2()`.
The `v1` version allows unbounded user input, while the `v2` version restricts the input to the length of a formatted date.

Inside the `main()` function, `v1` is used initially for `esittely` `v0.1.0`.
This is used as vulnerability source for `CVE-1900-33661` in the workshop.
Change it to the `v2` version and update the crate version to `0.1.1` in `Cargo.toml`
to fix the vulnerability in a semver-adhering version.
