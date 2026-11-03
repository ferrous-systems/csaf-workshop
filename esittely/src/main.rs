//! Mini-Game: Guess the Date
//!
//! Try to guess the secret date by entering a valid date in your terminal.
//! Your guess must follow the form: yyyy-mm-dd
//!
//! SPDX-License-Identifier: Apache-2.0
//! SPDX-FileCopyrightText: 2026 German Federal Office for Information Security (BSI) <https://www.bsi.bund.de>
//! Software-Engineering: 2026 Ferrous Systems GmbH <https://ferrous-systems.com>

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
