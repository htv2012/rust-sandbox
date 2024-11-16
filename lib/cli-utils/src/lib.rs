//! Command Line Iterface (CLI) Utilities
//! - read_stdin

use std::io::{BufRead, BufReader};

pub mod config;
pub mod color;

/// This function reads a line from stdin and return a String
/// It will panic upon failure
///
/// # Example
///
/// ```
/// use cli_utils::read_stdin;
/// let text = read_stdin()
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}


#[cfg(test)]
mod tests {
    use super::_read_stdin;
    use std::io::Cursor;

    #[test]
    fn test_read_input() {
        let input = "Hello, world\n";
        let expected = "Hello, world";
        let mut reader = Cursor::new(input);
        let actual = _read_stdin(&mut reader);
        assert_eq!(actual, expected);
    }
}
