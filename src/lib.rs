pub mod block_options;
pub mod console_color;
pub mod style_options;
pub mod table;

use crossterm::{event, event::{read, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode}, ExecutableCommand};
use std::io;
use std::io::{stdout, Read, Write};
use std::ptr::write;
use std::time::Duration;
use crossterm::style::{Color, Colors};
use crate::block_options::BlockOptions;
use crate::console_color::ConsoleColor;
use crate::style_options::StyleOptions;
use crate::table::Table;

pub struct RusticPrint {}

const CLEAR_COLOR: &str = "\x1b[0m";

impl RusticPrint {
    pub fn new() -> RusticPrint {
        RusticPrint {}
    }

    pub fn block(&self, message: &str) {
        let block_options = BlockOptions::default();
        self.fancy_block(message, block_options);
    }

    pub fn fancy_block(&self, message: &str, block_options: BlockOptions) {
        let mut formatted_text = String::new();
        let mut current_line = String::new();
        let prefix = block_options.prefix.unwrap_or("".to_string());
        let foreground = block_options
            .style
            .as_ref()
            .and_then(|style| style.foreground.as_ref().map(ConsoleColor::to_fg_ansi_code))
            .unwrap_or("");
        let background = block_options
            .style
            .as_ref()
            .and_then(|style| style.background.as_ref().map(ConsoleColor::to_bg_ansi_code))
            .unwrap_or("");
        let mut block_name = block_options.name.unwrap_or("".to_string());
        let mut first_line = true;
        let padding_line = " ".repeat(block_options.line_width - prefix.len());

        if block_options.padding {
            formatted_text.push_str(
                format!(
                    "{}{}{}{}{}\r\n",
                    foreground, background, prefix, padding_line, CLEAR_COLOR
                )
                .as_str(),
            );
        }

        for word in message.split_whitespace() {
            let mut line_prefix = prefix.clone();
            if first_line {
                line_prefix = format!("{}[{}] ", line_prefix, block_name);
            } else {
                line_prefix = format!("{}{}", line_prefix, " ".repeat(block_name.len() + 3));
            }

            let length = current_line.len() + word.len() + line_prefix.len();
            if length > block_options.line_width {
                if first_line {
                    first_line = false;
                }

                let trimmed_line = current_line.trim_end();
                let pushed_line = format!("{}{}", line_prefix, trimmed_line);

                let line_width = block_options.line_width as i32;
                let pushed_width = pushed_line.len() as i32;
                let mut finish_block = String::new();
                let how_many_spaces = line_width - pushed_width;

                if how_many_spaces > 0 {
                    finish_block.push_str(" ".repeat(how_many_spaces as usize).as_str());
                }

                formatted_text.push_str(
                    format!(
                        "{}{}{}{}{}\r\n",
                        foreground, background, pushed_line, finish_block, CLEAR_COLOR
                    )
                    .as_str(),
                );

                current_line.clear();
            }

            current_line.push_str(word);
            current_line.push_str(" ");
        }

        if !current_line.is_empty() {
            let line_prefix = format!("{}{}", prefix.clone(), " ".repeat(block_name.len() + 3));
            let pushed_line = format!("{}{}", line_prefix, current_line.trim_end());
            let line_width = block_options.line_width as i32;
            let pushed_width = pushed_line.len() as i32;
            let mut finish_block = String::new();
            let how_many_spaces = line_width - pushed_width;

            if how_many_spaces > 0 {
                finish_block.push_str(" ".repeat(how_many_spaces as usize).as_str());
            }

            formatted_text.push_str(
                format!(
                    "{}{}{}{}{}\r\n",
                    foreground, background, pushed_line, finish_block, CLEAR_COLOR
                )
                .as_str(),
            );
        }

        if block_options.padding {
            formatted_text.push_str(
                format!(
                    "{}{}{}{}{}\r\n",
                    foreground, background, prefix, padding_line, CLEAR_COLOR
                )
                .as_str(),
            );
        }

        print!(
            "{}{}",
            self.format_with_padding_lines(formatted_text.as_str()),
            CLEAR_COLOR
        );
    }

    pub fn title(&self, message: &str) {
        let underline = "=".repeat(message.len());
        let styled_text = format!("\x1b[32;1m{}\n{}\x1b[0m", message, underline); // Simple decoration

        println!("{}", self.format_with_padding_lines(styled_text.as_str()));
    }

    pub fn section(&self, message: &str) {
        let underline = "-".repeat(message.len());
        let styled_text = format!("\x1b[32;1m{}\n{}\x1b[0m", message, underline); // Simple decoration

        println!("{}", self.format_with_padding_lines(styled_text.as_str()));
    }

    pub fn listing(&self, elements: Vec<&str>) {
        let mut styled_text = String::from("\r\n");

        for element in elements {
            let element = element.trim();
            let list_element = format!("* {}", element);
            styled_text.push_str(
                format!("{}\r\n", self.format_with_padding(list_element.as_str(), 1)).as_str(),
            );
        }

        print!("{}", styled_text.as_str());
    }

    pub fn text(&self, message: &str) {
        print!(
            "{}",
            self.format_with_padding_lines(self.format_with_padding(message, 1).as_str())
        );
    }

    pub fn comment(&self, message: &str) {
        self.fancy_block(
            message,
            BlockOptions {
                prefix: Some(" //".to_string()),
                ..Default::default()
            },
        );
    }

    pub fn success(&self, message: &str) {
        self.fancy_block(
            message,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(ConsoleColor::Black),
                    background: Some(ConsoleColor::Green),
                }),
                prefix: Some(" ".to_string()),
                name: Some("OK".to_string()),
                padding: true,
                ..Default::default()
            },
        );
    }

    pub fn error(&self, message: &str) {
        self.fancy_block(
            message,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(ConsoleColor::White),
                    background: Some(ConsoleColor::Red),
                }),
                prefix: Some(" ".to_string()),
                name: Some("ERROR".to_string()),
                padding: true,
                ..Default::default()
            },
        );
    }

    pub fn warning(&self, message: &str) {
        self.fancy_block(
            message,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(ConsoleColor::Black),
                    background: Some(ConsoleColor::Yellow),
                }),
                prefix: Some(" ".to_string()),
                name: Some("WARNING".to_string()),
                padding: true,
                ..Default::default()
            },
        );
    }

    pub fn note(&self, message: &str) {
        self.fancy_block(
            message,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(ConsoleColor::Yellow),
                    background: None,
                }),
                prefix: Some(" ! ".to_string()),
                name: Some("NOTE".to_string()),
                padding: false,
                ..Default::default()
            },
        );
    }

    pub fn info(&self, message: &str) {
        self.fancy_block(
            message,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(ConsoleColor::Yellow),
                    background: None,
                }),
                prefix: Some(" ".to_string()),
                name: Some("INFO".to_string()),
                padding: false,
                ..Default::default()
            },
        );
    }

    pub fn caution(&self, message: &str) {
        self.fancy_block(
            message,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(ConsoleColor::White),
                    background: Some(ConsoleColor::Red),
                }),
                prefix: Some(" ! ".to_string()),
                name: Some("CAUTION".to_string()),
                padding: true,
                ..Default::default()
            },
        );
    }

    pub fn table(&self, headers: Vec<&str>, rows: Vec<Vec<&str>>) {
        let table = Table::new(headers, rows);

        table.print_table();
    }

    pub fn confirm(&self, question: &str, default: bool) -> bool {
        let mut stdout = io::stdout();

        // Enable raw mode
        enable_raw_mode().expect("Failed to enable raw mode");

        let mut has_written_line = false;

        has_written_line = true;
        let default_answer = if default { "yes" } else { "no" };
        let formatted_question = format!(
            "{}{} (yes/no) {}[{}{}{}]:\r\n > ",
            ConsoleColor::Green.to_fg_ansi_code(),
            question,
            CLEAR_COLOR,
            ConsoleColor::Yellow.to_fg_ansi_code(),
            default_answer,
            CLEAR_COLOR
        );
        write!(stdout, "{}", formatted_question).expect("Failed to write to stdout");
        stdout.flush().expect("Failed to flush stdout");

        let mut input = String::new();

        loop {
            // Read the user input character by character
            if let Event::Key(key_event) = read().expect("Failed to read event") {
                match key_event.code {
                    KeyCode::Char(c) => {
                        print!("{}", c);
                        input.push(c);
                    }
                    KeyCode::Enter => {
                        println!();
                        break;
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            print!("\x08 \x08"); // Handle backspace visually
                        }
                    }
                    _ => {}
                }
                stdout.flush().expect("Failed to flush stdout");
            }
        }

        disable_raw_mode().expect("Failed to disable raw mode");

        println!();

        if input.is_empty() {
            return default;
        } else if !input.is_empty() && (input.eq("yes") || input.eq("y")) {
            return true;
        }

        return false;
    }

    fn format_with_padding_lines(&self, message: &str) -> String {
        let formatted_text = format!("\r\n{}\r\n", message);

        formatted_text
    }

    fn format_with_padding(&self, message: &str, padding: usize) -> String {
        let padding = " ".repeat(padding);

        format!("{}{}", padding, message)
    }

    pub fn ask(&self, question: &str, default: Option<&str>, validator: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
    )
        -> String
    {
        let mut stdout = io::stdout();

        loop {
            Self::ask_question(question, default);
            stdout.flush().expect("Failed to flush stdout");

            let mut input = String::new();

            // Read user input
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let input = input.trim();

            // Use default value if input is empty
            let answer = if input.is_empty() {
                default.unwrap_or("").to_string()
            } else {
                input.to_string()
            };

            // Validate the input
            if let Some(ref validate) = validator {
                match validate(&answer) {
                    Ok(_) => return answer,
                    Err(err) => {
                        println!("\x1b[31m{}\x1b[0m", err); // Red color for error messages
                    }
                }
            } else {
                return answer;
            }
        }
    }

    fn ask_question(question: &str, default_text: Option<&str>) {
        let default_text = if let Some(default_text) = default_text {
            format!(" [{}]", default_text)
        } else {
            String::new()
        };

        print!(
            "{}{}{}:{}\n> ",
            "\x1b[32m", // Green for the question
            question,
            default_text,
            "\x1b[0m", // Reset color for the default
        );
    }

    fn choose(question: &str, choices: &[&str]) -> String {
        let mut stdout = io::stdout();
        let mut selected: usize = 0;

        print_choices(choices, selected);

        loop {
            // crossterm’s `poll` checks if an event is ready without blocking forever
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    match key_event.code {
                        KeyCode::Up => {
                            // Move selection up (with wrap-around)
                            if selected == 0 {
                                selected = choices.len() - 1;
                            } else {
                                selected -= 1;
                            }
                            // Print the list again on new lines
                            print_choices(choices, selected);
                        }
                        KeyCode::Down => {
                            // Move selection down (with wrap-around)
                            selected = (selected + 1) % choices.len();
                            // Print the list again on new lines
                            print_choices(choices, selected);
                        }
                        KeyCode::Enter => {
                            // Finalize the selection
                            disable_raw_mode().unwrap();
                            return choices[selected].to_string();
                        }
                        KeyCode::Esc => {
                            // Optional: handle ESC as a “cancel”
                            disable_raw_mode().unwrap();
                            return String::new();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}