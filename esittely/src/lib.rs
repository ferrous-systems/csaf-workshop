use std::time::Duration;

use time::format_description::BorrowedFormatItem;

const MAX_YEAR: i32 = 9999;

/// The expected date format used to parse a given guess.
static EXPECTED_DATE_FORMAT: &[BorrowedFormatItem<'static>] =
    time::macros::format_description!("[year padding:none]-[month]-[day]");

/// Starts the date guessing.
///
/// Intermediary information about the game state is sent to the receiver of the given message sender.
/// The `retrieve_guess`
pub fn guess_date<A: Send + Sync>(
    msg_sender: std::sync::mpsc::Sender<String>,
    retrieve_guess: fn(&A) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>,
    retrieve_args: &A,
    guess_timeout: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    msg_sender.send("Your guess must follow the form: yyyy-mm-dd".to_string())?;
    msg_sender.send("e.g. 2026-10-13".to_string())?;
    msg_sender.send(String::new())?;
    msg_sender.send(format!("Years are in the range: 1..={MAX_YEAR}"))?;
    msg_sender.send("Years before 1000, no padding is needed".to_string())?;
    msg_sender.send("e.g. 100-10-13".to_string())?;
    msg_sender.send("---------------------------------".to_string())?;

    let secret = secret_date();
    let start = std::time::Instant::now();

    loop {
        msg_sender.send("Enter guess:".to_string())?;

        let (tx, rx) = std::sync::mpsc::channel();

        let args = retrieve_args;
        let line = std::thread::scope(|s| {
            s.spawn(move || {
                let result = retrieve_guess(args);
                let _ = tx.send(result.map(|line| line.trim().to_owned()));
            });

            let line = match rx.recv_timeout(guess_timeout) {
                Ok(Ok(line)) => line,
                Ok(Err(err)) => {
                    return Err::<std::string::String, Box<dyn std::error::Error>>(
                        format!("Read input error: {err}").into(),
                    );
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    return Err("Time out while waiting for user input".into());
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    return Err("Reading user input for new guess failed".into());
                }
            };

            Ok(line)
        })?;

        match time::Date::parse(&line, EXPECTED_DATE_FORMAT) {
            Ok(guess) => {
                if guess.eq(&secret) {
                    let elapsed = start.elapsed();

                    msg_sender.send("Correct guess!!!".into())?;
                    msg_sender.send(format!("It took you: {}s", elapsed.as_secs()))?;
                    return Ok(());
                } else if guess.lt(&secret) {
                    msg_sender.send("--- Later ---".into())?;
                } else {
                    msg_sender.send("-- Earlier --".into())?;
                }
            }
            Err(err) => {
                msg_sender.send(format!("Invalid date format! Error: {err}"))?;
                msg_sender.send("Expected valid date in form: yyyy-mm-dd".into())?;
            }
        }

        msg_sender.send("---------------------------------".into())?;
    }
}

/// Constructs a random valid date with the year being in range 1..=[`MAX_YEAR`].
fn secret_date() -> time::Date {
    let mut secret = time::Date::from_calendar_date(2026, time::Month::October, 13)
        .expect("Default date is always valid");

    let mut upper_year = MAX_YEAR;
    let mut lower_year = 1;
    loop {
        if let Ok(new_secret) = secret.replace_year(rand::random_range(lower_year..=upper_year)) {
            secret = new_secret;
            break;
        }

        if upper_year > lower_year {
            lower_year += 1;
            upper_year -= 1;
        } else {
            break;
        }
    }

    let mut upper_month = 12;
    let mut lower_month = 1;
    loop {
        if let Ok(month) = time::Month::try_from(rand::random_range(lower_month..=upper_month))
            && let Ok(new_secret) = secret.replace_month(month)
        {
            secret = new_secret;
            break;
        }

        if upper_month > lower_month {
            lower_month += 1;
            upper_month -= 1;
        } else {
            break;
        }
    }

    let mut upper_day = 31;
    let mut lower_day = 1;
    loop {
        if let Ok(new_secret) = secret.replace_day(rand::random_range(lower_day..=upper_day)) {
            secret = new_secret;
            break;
        }

        if upper_day > lower_day {
            lower_day += 1;
            upper_day -= 1;
        } else {
            break;
        }
    }

    secret
}
