use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum TermColor {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl fmt::Display for TermColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TermColor::Reset => write!(f, "\x1B[0m"),
            TermColor::Black => write!(f, "\x1B[30m"),
            TermColor::Red => write!(f, "\x1B[31m"),
            TermColor::Green => write!(f, "\x1B[32m"),
            TermColor::Yellow => write!(f, "\x1B[33m"),
            TermColor::Blue => write!(f, "\x1B[34m"),
            TermColor::Magenta => write!(f, "\x1B[35m"),
            TermColor::Cyan => write!(f, "\x1B[36m"),
            TermColor::White => write!(f, "\x1B[37m"),
        }
    }
}

impl TermColor {
    pub fn apply(&self, text: &str) -> String {
        format!("{}{}{}", self, text, TermColor::Reset)
    }
}
