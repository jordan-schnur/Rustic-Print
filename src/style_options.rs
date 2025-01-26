use crossterm::style::{Color, Colors};
use crate::console_color::ConsoleColor;

pub struct StyleOptions {
    pub(crate) foreground: Option<ConsoleColor>,
    pub(crate) background: Option<ConsoleColor>,
}
