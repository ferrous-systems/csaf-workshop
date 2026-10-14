/// Retrieve a new guess from `stdin`.
/// Reads until a newline symbol has been detected.
pub fn retrieve_guess_unbounded(_args: &()) -> Result<String, Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut line = String::new();

    let result = stdin.read_line(&mut line);
    result
        .map(|_| line.trim().to_owned())
        .map_err(|err| Box::new(err).into())
}

/// Retrieve a new guess from `stdin`.
/// Reads until a newline symbol has been detected, or the number of read bytes exceeds the limit of a valid date guess.
pub fn retrieve_guess_bounded(
    max_input_length: &usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();

    read_bounded_line(&mut input, *max_input_length)
}

/// Read from `stdin` until a newline is detected, or the number of read bytes exceeds `max_input_length`.
fn read_bounded_line<R: std::io::Read>(
    reader: &mut R,
    max_input_length: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut bytes = Vec::with_capacity(max_input_length);
    let mut byte = [0u8; 1];

    loop {
        match reader.read(&mut byte)? {
            0 if bytes.is_empty() => {
                return Err("Input closed without a guess entered".into());
            }
            0 => break,
            _ if byte[0] == b'\n' => break,
            _ => {
                if bytes.len() >= max_input_length {
                    return Err("Your guess exceeded the maximum length".into());
                }

                bytes.push(byte[0]);
            }
        }
    }

    // Windows newline '\r\n'
    if bytes.last() == Some(&b'\r') {
        bytes.pop();
    }

    String::from_utf8(bytes)
        .map_err(|err| format!("Your input is invalid UTF-8. Error: {err}").into())
        .map(|s| s.trim().to_owned())
}
