pub mod block_options;
pub mod style_options;
pub mod table;
mod messages;

use std::cmp::min;
use crossterm::{event, event::{read, Event, KeyCode}, execute, queue, style::{Color, Stylize, SetForegroundColor, SetBackgroundColor, ResetColor}, terminal, terminal::{disable_raw_mode, enable_raw_mode}, ExecutableCommand, QueueableCommand};
use std::io::{self, stdout, Read, Write};
use std::time::Duration;
use crossterm::cursor::MoveTo;
use crossterm::style::{style, Attribute, Print, PrintStyledContent, SetColors, StyledContent};
use textwrap::{fill, wrap, Options};
use crate::block_options::{BlockOptions};
use crate::messages::Messages;
use crate::style_options::StyleOptions;
use crate::table::Table;

pub struct RusticPrint {}


// TODO: Add support for macros
impl RusticPrint {
    pub fn new() -> RusticPrint {
        RusticPrint {}
    }

    /// Prints a simple block with the default block options.
    // pub fn block(&self, message: &str) {
    //     let block_options = BlockOptions::default();
    //     self.fancy_block(message, block_options);
    // }

    pub fn fancy_block<T>(&self, message: T, block_options: BlockOptions)
        -> Result<(), Box<dyn std::error::Error>>
    where T: Into<Messages>{
        let message = message.into();

        let mut stdout = stdout();

        let term_width = terminal::size().unwrap_or((120, 0)).0 as usize;
        let mut wrap_width = if term_width > 120 { 120 } else { term_width };
        if cfg!(windows) {
            wrap_width = wrap_width.saturating_sub(1);
        }

        let has_block_type = block_options.block_type.is_some();
        let default_block_type = "".to_string();
        let block_type = block_options.block_type.unwrap_or("".to_string());
        let block_length = block_type.clone().len();
        // Text starts at prefix + block type + 5 for the two brackets and the space between them
        let extra_space = if block_options.prefix.is_empty() { 2 } else { 2 };
        let text_start = block_options.prefix.len()
            + block_length
            + extra_space;

        queue!(stdout, Print("\n"))?;

        if block_options.padding {
            if let Some(style) = &block_options.style {
                if let Some(bg) = style.background {
                    let padding_line = " ".repeat(wrap_width).on(bg);
                    queue!(stdout, PrintStyledContent(padding_line), ResetColor, Print("\r\n"))?;
                } else {
                    queue!(stdout, Print("\n"))?;
                }
            } else {
                queue!(stdout, Print("\n"))?;
            }
        }

        // Build combined entire message string

        match message {
            Messages::Single(ref message) => {
                let mut whole_message = message.clone();
                let mut initial_indent = block_options.prefix.clone();
                let mut prefix = block_options.prefix.clone();


                if (initial_indent.is_empty()) {
                    initial_indent = " ".to_string();
                }

                if has_block_type {
                    initial_indent = format!("{}[{}] ", initial_indent, block_type);
                }

                let initial_line_offset = initial_indent.len();
                let subsequent_indent = format!("{}{}", prefix, " ".repeat(initial_line_offset - prefix.len()));
                let options = Options::new(wrap_width)
                    .initial_indent(&*initial_indent)
                    .subsequent_indent(&*subsequent_indent);

                fill(&whole_message, &options)
                    .lines()
                    .for_each(|line| {
                        let line_len = line.clone().len();
                        let end_padding_text = " ".repeat(wrap_width.saturating_sub(line.len()));

                        let mut text = style(format!("{}{}", line, end_padding_text));
                            if let Some(style) = &block_options.style {
                                if let Some(bg) = style.background {
                                    text = text.on(bg);
                                }

                                if let Some(fg) = style.foreground {
                                    text = text.with(fg);
                                }
                            }

                            queue!(stdout, PrintStyledContent(text), ResetColor, Print("\n")).unwrap();
                    });
            }
            Messages::Multiple(ref messages) => {
                let mut whole_message = message.clone();
                let mut initial_indent = block_options.prefix.clone();
                let mut prefix = block_options.prefix.clone();


                if (initial_indent.is_empty()) {
                    initial_indent = " ".to_string();
                }

                if has_block_type {
                    initial_indent = format!("{}[{}] ", initial_indent, block_type);
                }

                let initial_line_offset = initial_indent.len();
                let subsequent_indent = format!("{}{}", prefix, " ".repeat(initial_line_offset - prefix.len()));
                let initial_options = Options::new(wrap_width)
                    .initial_indent(&*initial_indent)
                    .subsequent_indent(&*subsequent_indent);

                let mut first_message = true;

                for (i, message) in messages.iter().enumerate() {
                    let mut options = initial_options.clone();
                    if first_message {
                        first_message = false;
                    } else {
                        if let Some(style) = &block_options.style {
                            if let Some(bg) = style.background {
                                let padding_line = " ".repeat(wrap_width).on(bg);
                                queue!(stdout, PrintStyledContent(padding_line), ResetColor, Print("\r\n"))?;
                            } else {
                                queue!(stdout, Print("\n"))?;
                            }
                        } else {
                            queue!(stdout, Print("\n"))?;
                        }

                        options = options.clone().initial_indent(&*subsequent_indent);
                    }

                    fill(&message, &options)
                        .lines()
                        .for_each(|line| {
                            let line_len = line.clone().len();
                            let end_padding_text = " ".repeat(wrap_width.saturating_sub(line.len()));

                            let mut text = style(format!("{}{}", line, end_padding_text));
                            if let Some(style) = &block_options.style {
                                if let Some(bg) = style.background {
                                    text = text.on(bg);
                                }

                                if let Some(fg) = style.foreground {
                                    text = text.with(fg);
                                }
                            }

                            queue!(stdout, PrintStyledContent(text), ResetColor, Print("\n")).unwrap();
                        });
                }
            }
        }

        if block_options.padding {
            if let Some(style) = &block_options.style {
                if let Some(bg) = style.background {
                    let padding_line = " ".repeat(wrap_width).on(bg);
                    queue!(stdout, PrintStyledContent(padding_line), ResetColor, Print("\r\n"))?;
                } else {
                    queue!(stdout, Print("\n"))?;
                }
            } else {
                queue!(stdout, Print("\n"))?;
            }
        }

        queue!(stdout, Print("\n"))?;


        // Flush the queue to render the block in one go.
        stdout.flush()?;

        Ok(())
    }

//     /// Prints a title in bold green with an underline.
//     pub fn title(&self, message: &str) {
//         let underline = "=".repeat(message.len());
//         // Using the Stylize trait to add colors and bold style.
//         let styled_text = format!(
//             "\n{}\n{}",
//             message.green().bold(),
//             underline.green().bold()
//         );
//         println!("{}", styled_text);
//     }
//
//     /// Prints a section header in bold green with a dashed underline.
//     pub fn section(&self, message: &str) {
//         let underline = "-".repeat(message.len());
//         let styled_text = format!(
//             "\n{}\n{}",
//             message.green().bold(),
//             underline.green().bold()
//         );
//         println!("{}", styled_text);
//     }
//
//     /// Prints a list of elements with a bullet.
//     pub fn listing(&self, elements: Vec<&str>) {
//         let mut styled_text = String::from("\n");
//         for element in elements {
//             let element = element.trim();
//             let list_element = format!("* {}", element);
//             styled_text.push_str(&format!("{}\n", self.format_with_padding(&list_element, 1)));
//         }
//         print!("{}", styled_text);
//     }
//
//     /// Prints plain text with a blank line before and after.
//     pub fn text(&self, message: &str) {
//         print!("{}", self.format_with_padding_lines(&self.format_with_padding(message, 1)));
//     }
//
//     /// Prints a comment-styled block.
//     pub fn comment(&self, message: &str) {
//         self.fancy_block(
//             message,
//             BlockOptions {
//                 prefix: Some(" //".to_string()),
//                 ..Default::default()
//             },
//         );
//     }
//

    pub fn success_multiple(&self, messages: Vec<&str>) {
        self.fancy_block(
            Messages::Multiple(messages.iter().map(|s| s.to_string()).collect()),
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::Black),
                    background: Some(Color::DarkGreen),
                }),
                block_type: Some("OK".to_string()),
                padding: true,
                ..Default::default()
            },
        ).expect("Failed to print success block");
    }

    /// Prints a success block with black text on a green background.
    pub fn success(&self, message: &str) {
        self.fancy_block(
            Messages::Single(message.to_string()),
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::Black),
                    background: Some(Color::DarkGreen),
                }),
                block_type: Some("OK".to_string()),
                padding: true,
                ..Default::default()
            },
        ).expect("Failed to print success block");
    }

    pub fn caution<T>(&self, messages: T)
    where T: Into<Messages>{
        self.fancy_block(
            messages,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::Black),
                    background: Some(Color::DarkGreen),
                }),
                block_type: Some("OK".to_string()),
                padding: true,
                ..Default::default()
            },
        ).expect("Failed to print caution block");
    }
//
//     /// Prints an error block with white text on a red background.
//     pub fn error(&self, message: &str) {
//         self.fancy_block(
//             message,
//             BlockOptions {
//                 style: Some(StyleOptions {
//                     foreground: Some(Color::White),
//                     background: Some(Color::Red),
//                 }),
//                 prefix: Some(" ".to_string()),
//                 name: Some("ERROR".to_string()),
//                 padding: true,
//                 ..Default::default()
//             },
//         );
//     }
//
//     /// Prints a warning block with black text on a yellow background.
//     pub fn warning(&self, message: &str) {
//         self.fancy_block(
//             message,
//             BlockOptions {
//                 style: Some(StyleOptions {
//                     foreground: Some(Color::Black),
//                     background: Some(Color::Yellow),
//                 }),
//                 prefix: Some(" ".to_string()),
//                 name: Some("WARNING".to_string()),
//                 padding: true,
//                 ..Default::default()
//             },
//         );
//     }
//
//     /// Prints a note block with yellow text.
//     pub fn note(&self, message: &str) {
//         self.fancy_block(
//             message,
//             BlockOptions {
//                 style: Some(StyleOptions {
//                     foreground: Some(Color::Yellow),
//                     background: None,
//                 }),
//                 prefix: Some(" ! ".to_string()),
//                 name: Some("NOTE".to_string()),
//                 padding: false,
//                 ..Default::default()
//             },
//         );
//     }
//
    pub fn info_multiple(&self, messages: Vec<&str>) {
        self.fancy_block(
            Messages::Multiple(messages.iter().map(|s| s.to_string()).collect()),
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::Green),
                    background: None,
                }),
                block_type: Some("INFO".to_string()),
                padding: true,
                ..Default::default()
            },
        ).expect("Failed to print info block");
    }

    /// Prints an info block with yellow text.
    pub fn info(&self, message: &str) {
        self.fancy_block(
            Messages::Single(message.to_string()),
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::Green),
                    background: None,
                }),
                block_type: Some("INFO".to_string()),
                padding: true,
                ..Default::default()
            },
        ).expect("Failed to print info block");
    }
//
//     /// Prints a table.
//     pub fn table(&self, headers: Vec<&str>, rows: Vec<Vec<&str>>) {
//         let table = Table::new(headers, rows);
//         table.print_table();
//     }
//
//     /// Prompts the user for confirmation. Uses raw mode and crossterm styling.
//     pub fn confirm(&self, question: &str, default: bool) -> bool {
//         let mut stdout = io::stdout();
//         enable_raw_mode().expect("Failed to enable raw mode");
//
//         let default_answer = if default { "yes" } else { "no" };
//
//         // Using the Stylize trait to color the prompt.
//         print!(
//             "{} (yes/no) [{}]:\r\n > ",
//             question.green(),
//             default_answer.yellow()
//         );
//         stdout.flush().expect("Failed to flush stdout");
//
//         let mut input = String::new();
//
//         loop {
//             if let Event::Key(key_event) = read().expect("Failed to read event") {
//                 match key_event.code {
//                     KeyCode::Char(c) => {
//                         print!("{}", c);
//                         input.push(c);
//                     }
//                     KeyCode::Enter => {
//                         println!();
//                         break;
//                     }
//                     KeyCode::Backspace => {
//                         if !input.is_empty() {
//                             input.pop();
//                             print!("\x08 \x08"); // Visual backspace.
//                         }
//                     }
//                     _ => {}
//                 }
//                 stdout.flush().expect("Failed to flush stdout");
//             }
//         }
//
//         disable_raw_mode().expect("Failed to disable raw mode");
//         println!();
//
//         if input.trim().is_empty() {
//             default
//         } else if input.trim().eq_ignore_ascii_case("yes")
//             || input.trim().eq_ignore_ascii_case("y")
//         {
//             true
//         } else {
//             false
//         }
//     }
//
//     fn format_with_padding_lines(&self, message: &str) -> String {
//         format!("\n{}\n", message)
//     }
//
//     fn format_with_padding(&self, message: &str, padding: usize) -> String {
//         format!("{}{}", " ".repeat(padding), message)
//     }
//
//     /// Prompts the user with a question (and optional default/validator) and returns the answer.
//     pub fn ask(
//         &self,
//         question: &str,
//         default: Option<&str>,
//         validator: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
//     ) -> String {
//         let mut stdout = io::stdout();
//
//         loop {
//             Self::ask_question(question, default);
//             stdout.flush().expect("Failed to flush stdout");
//
//             let mut input = String::new();
//             io::stdin().read_line(&mut input).expect("Failed to read line");
//             let input = input.trim();
//
//             // Use the default value if the user provides no input.
//             let answer = if input.is_empty() {
//                 default.unwrap_or("").to_string()
//             } else {
//                 input.to_string()
//             };
//
//             // Validate the answer if a validator was provided.
//             if let Some(ref validate) = validator {
//                 match validate(&answer) {
//                     Ok(_) => return answer,
//                     Err(err) => {
//                         println!("{}", err.red());
//                     }
//                 }
//             } else {
//                 return answer;
//             }
//         }
//     }
//
//     fn ask_question(question: &str, default_text: Option<&str>) {
//         let default_text = if let Some(dt) = default_text {
//             format!(" [{}]", dt)
//         } else {
//             String::new()
//         };
//
//         print!("{}{}:\n> ", question.green(), default_text);
//     }
//
//     /// Allows the user to choose an option from a list using the arrow keys.
//     /// (Press Enter to select; Esc cancels.)
//     fn choose(question: &str, choices: &[&str]) -> String {
//         let stdout = io::stdout();
//         let mut selected: usize = 0;
//
//         // Print the question.
//         println!("{}", question.green().bold());
//         print_choices(choices, selected);
//
//         loop {
//             if event::poll(Duration::from_millis(50)).unwrap() {
//                 if let Event::Key(key_event) = event::read().unwrap() {
//                     match key_event.code {
//                         KeyCode::Up => {
//                             if selected == 0 {
//                                 selected = choices.len() - 1;
//                             } else {
//                                 selected -= 1;
//                             }
//                             print_choices(choices, selected);
//                         }
//                         KeyCode::Down => {
//                             selected = (selected + 1) % choices.len();
//                             print_choices(choices, selected);
//                         }
//                         KeyCode::Enter => {
//                             disable_raw_mode().unwrap();
//                             return choices[selected].to_string();
//                         }
//                         KeyCode::Esc => {
//                             disable_raw_mode().unwrap();
//                             return String::new();
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }
//
//     /// Helper: Prints a single line using the given foreground and background colors.
//     fn print_styled_line(&self, line: &str, fg: Option<Color>, bg: Option<Color>) {
//         let styled_line = match (fg, bg) {
//             (Some(f), Some(b)) => line.with(f).on(b),
//             (Some(f), None) => line.with(f),
//             (None, Some(b)) => line.on(b),
//             (None, None) => style(line), // Instead of line.normal()
//         };
//         println!("{}", styled_line);
//     }
// }
//
// /// Helper function for the `choose` method: prints the choices with the current selection highlighted.
// fn print_choices(choices: &[&str], selected: usize) {
//     // For simplicity, reprint all choices.
//     // The selected option is rendered with inverted colors.
//     for (i, choice) in choices.iter().enumerate() {
//         if i == selected {
//             // For the selected option, use a black-on-white style.
//             println!("{}", format!("> {}", choice).black().on_white());
//         } else {
//             println!("  {}", choice);
//         }
//     }
}

fn escape_text(input: &str) -> String {
    input.replace("\x1B", "\\x1B")
}