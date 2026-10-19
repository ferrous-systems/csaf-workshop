use time::format_description::BorrowedFormatItem;

pub mod retrieval;

const MAX_YEAR: i32 = 9999;

/// The expected date format used to parse a given guess.
static EXPECTED_DATE_FORMAT: &[BorrowedFormatItem<'static>] =
    time::macros::format_description!("[year padding:none]-[month]-[day]");

/// Initial version for playing "Guess the Date".
///
/// It allows unbounded input for a date guess.
pub fn guess_date_v1() -> Result<(), Box<dyn std::error::Error>> {
    guess_date(retrieval::retrieve_guess_unbounded, &())
}

/// Fixed version for playing "Guess the Date".
///
/// It restricts the input for a date guess to stay within the range for the maximum possible date.
pub fn guess_date_v2() -> Result<(), Box<dyn std::error::Error>> {
    let max_guess_len = "9999-12-31".len() + 1; // +1 for optional carriage return
    guess_date(retrieval::retrieve_guess_bounded, &max_guess_len)
}

/// Starts the date guessing.
///
/// Intermediary information about the game state is sent to the receiver of the given message sender.
/// The `retrieve_guess` function is called to retrieve a new guess on every turn.
/// Custom arguments `retrieve_args` are passed through to the `retrieve_guess` function.
/// The `guess_timeout` is used to raise a timeout if no new guess has been entered.
fn guess_date<A>(
    retrieve_guess: fn(&A) -> Result<String, Box<dyn std::error::Error>>,
    retrieve_args: &A,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("New game! Guess the Date");
    println!("Your guess must follow the form: yyyy-mm-dd");
    println!("e.g. 2026-10-13");
    println!();
    println!("Years are in the range: 1..={MAX_YEAR}");
    println!("Years before 1000, no padding is needed");
    println!("e.g. 100-10-13");
    println!("---------------------------------");

    let secret = secret_date();
    let start = std::time::Instant::now();

    loop {
        println!("Enter guess:");

        let line = retrieve_guess(retrieve_args)?;

        match time::Date::parse(&line, EXPECTED_DATE_FORMAT) {
            Ok(guess) => {
                if guess.eq(&secret) {
                    let elapsed = start.elapsed();

                    println!("Correct guess!!!");
                    println!("It took you: {}s", elapsed.as_secs());
                    return Ok(());
                } else if guess.lt(&secret) {
                    println!("--- Later ---");
                } else {
                    println!("-- Earlier --");
                }
            }
            Err(err) => {
                println!("Invalid date format! Error: {err}");
                println!("Expected valid date in form: yyyy-mm-dd");
            }
        }

        println!("---------------------------------");
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
