//! Colorize output for terminal via ANSI escape codes
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}", red("this text is in red"));
//! ```

/// Returns a string with the ANSI escape code for red
/// # Examples
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("this text is in red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for green
/// # Examples
/// ```
/// use cli_utils::colors::*;
/// println!("{}", green("this text is in green"));
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for blue
/// # Examples
/// ```
/// use cli_utils::colors::*;
/// println!("{}", blue("this text is in blue"));
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for bold (bright).
/// # Examples
/// ```
/// use cli_utils::colors::*;
/// println!("{}", bold("this text is in bold"));
///
/// // Combine with other colors
/// println!("{}", bold(&red("Error: Out of coffee")));
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for resetting the colors back to normal.
/// # Examples
/// ```
/// use cli_utils::colors::*;
/// println!("{}", reset("this text is in normal color"));
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// Represents different colors
pub enum Color {
    Red,
    Green,
    Blue,
    Bold,
}

/// Represent a string with embedded ANSI color code
pub struct ColorString {
    /// The color, see [Color]
    pub color: Color,

    /// The text itself
    pub string: String,

    /// The colorized text
    pub colorized: String,
}

impl ColorString {
    /// Create colorized version of the text base on `color` and `string`
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    /// Reset the color to normal
    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }
}
