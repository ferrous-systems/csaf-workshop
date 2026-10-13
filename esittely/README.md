# `esittely` - Guess the Date

This crate implements the "Guess the Date" game.
It can be started in a terminal or integrated into your application.

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

## Integrate in your App

The game logic is extracted into the `esittely::guess_date()` function that can be integrated into your crate.
This could, for example, be an online service that lets people play "Guess the Date" on a website.
