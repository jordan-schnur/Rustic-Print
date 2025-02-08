pub mod block_options;
mod messages;
pub mod style_options;
pub mod table;

use crate::block_options::BlockOptions;
use crate::messages::Messages;
use crate::style_options::StyleOptions;
use crate::table::Table;
use crossterm::cursor::MoveTo;
use crossterm::style::{style, Attribute, Print, PrintStyledContent, SetColors, StyledContent};
use crossterm::{
    event,
    event::{read, Event, KeyCode},
    execute, queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};
use std::cmp::min;
use std::io::{self, stdout, Read, Write};
use std::time::Duration;
use textwrap::{fill, wrap, Options};

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

    pub fn fancy_block<T>(
        &self,
        message: T,
        block_options: BlockOptions,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Into<Messages>,
    {
        let message = message.into();
        let mut stdout = stdout();

        // Determine terminal width (default to 120 if unavailable) and cap the wrap width.
        let term_width = terminal::size().unwrap_or((120, 0)).0 as usize;
        let mut wrap_width = if term_width > 120 { 120 } else { term_width };
        if cfg!(windows) {
            wrap_width = wrap_width.saturating_sub(1);
        }

        // Start with an empty line.
        queue!(stdout, Print("\n"))?;

        // Prepare effective prefix (default to a single space if empty)
        let mut prefix = block_options.prefix.clone();
        if prefix.is_empty() {
            prefix = " ".to_string();
        }

        // Print top padding if enabled.
        if block_options.padding {
            print_padding_line(&mut stdout, wrap_width, &block_options, &prefix)?;
        }

        // Prepare indent strings.
        let block_type = block_options.block_type.clone().unwrap_or_default();
        let initial_indent = if !block_type.is_empty() {
            format!("{}[{}] ", prefix, block_type)
        } else {
            prefix.clone()
        };
        let subsequent_indent = format!(
            "{}{}",
            prefix,
            " ".repeat(initial_indent.len().saturating_sub(prefix.len()))
        );

        // Convert the message into a vector of strings.
        let messages_vec: Vec<String> = match message {
            Messages::Single(ref msg) => vec![msg.clone()],
            Messages::Multiple(ref msgs) => msgs.clone(),
        };

        for (i, msg) in messages_vec.iter().enumerate() {
            if i > 0 {
                print_padding_line(&mut stdout, wrap_width, &block_options, &prefix)?;
            }

            // For the first message, use the full initial indent; for others, use the subsequent indent.
            let effective_options = if i == 0 {
                Options::new(wrap_width)
                    .initial_indent(&initial_indent)
                    .subsequent_indent(&subsequent_indent)
            } else {
                Options::new(wrap_width)
                    .initial_indent(&subsequent_indent)
                    .subsequent_indent(&subsequent_indent)
            };

            // Wrap and print each line of the message.
            for line in fill(msg, &effective_options).lines() {
                styled_print_line(&mut stdout, line, wrap_width, &block_options)?;
            }
        }

        // Print bottom padding if enabled.
        if block_options.padding {
            print_padding_line(&mut stdout, wrap_width, &block_options, &prefix)?;
        }

        queue!(stdout, Print("\n"))?;
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
        )
        .expect("Failed to print success block");
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
        )
        .expect("Failed to print success block");
    }

    pub fn caution<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.fancy_block(
            messages,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::Grey),
                    background: Some(Color::DarkRed),
                }),
                block_type: Some("CAUTION".to_string()),
                prefix: " ! ".to_string(),
                padding: true,
                ..Default::default()
            },
        )
        .expect("Failed to print caution block");
    }

    pub fn error<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.fancy_block(
            messages,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::Grey),
                    background: Some(Color::DarkRed),
                }),
                block_type: Some("ERROR".to_string()),
                prefix: " ".to_string(),
                padding: true,
                ..Default::default()
            },
        )
        .expect("Failed to print error block");
    }

    pub fn comment<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.fancy_block(
            messages,
            BlockOptions {
                prefix: " // ".to_string(),
                ..Default::default()
            },
        )
        .expect("Failed to print comment block");
    }

    pub fn warning<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.fancy_block(
            messages,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::Black),
                    background: Some(Color::DarkYellow),
                }),
                block_type: Some("WARNING".to_string()),
                padding: true,
                ..Default::default()
            },
        )
        .expect("Failed to print comment block");
    }

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
        )
        .expect("Failed to print info block");
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
        )
        .expect("Failed to print info block");
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

/// Prints a styled padding line that includes the provided prefix.
///
/// The line is constructed by concatenating the prefix and spaces to fill the wrap width.
/// Styling (background) is applied if specified in `block_options.style`.
///
/// # Arguments
///
/// * `stdout` - The mutable writer for stdout.
/// * `wrap_width` - The width used to compute the padding.
/// * `block_options` - Options that may contain style information.
/// * `prefix` - The prefix to include at the beginning of the line.
///
/// # Returns
///
/// A `Result` indicating success or any encountered error.
fn print_padding_line(
    stdout: &mut impl Write,
    wrap_width: usize,
    block_options: &BlockOptions,
    prefix: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let line = if wrap_width > prefix.len() {
        format!("{}{}", prefix, " ".repeat(wrap_width - prefix.len()))
    } else {
        prefix.to_string()
    };

    if let Some(style_cfg) = &block_options.style {
        let mut styled_line = style(line);

        if let Some(bg) = style_cfg.background {
            styled_line = styled_line.on(bg);
        }

        if let Some(fg) = style_cfg.foreground {
            styled_line = styled_line.with(fg);
        }

        queue!(
            stdout,
            PrintStyledContent(styled_line),
            ResetColor,
            Print("\r\n")
        )?;
        return Ok(());
    }
    queue!(stdout, Print(line), Print("\r\n"))?;
    Ok(())
}

/// Styles and prints a single line with appropriate end padding.
///
/// The line is padded with spaces to ensure it spans the full `wrap_width`, and styling (background
/// and foreground colors) is applied if specified in `block_options`.
///
/// # Arguments
///
/// * `stdout` - The mutable writer for stdout.
/// * `line` - The text line to be styled and printed.
/// * `wrap_width` - The total width for the line (used for padding).
/// * `block_options` - Options that may contain styling information.
///
/// # Returns
///
/// A `Result` indicating success or any encountered error.
fn styled_print_line(
    stdout: &mut impl Write,
    line: &str,
    wrap_width: usize,
    block_options: &BlockOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let end_padding = " ".repeat(wrap_width.saturating_sub(line.len()));
    let mut styled = style(format!("{}{}", line, end_padding));
    if let Some(style_cfg) = &block_options.style {
        if let Some(bg) = style_cfg.background {
            styled = styled.on(bg);
        }
        if let Some(fg) = style_cfg.foreground {
            styled = styled.with(fg);
        }
    }
    queue!(stdout, PrintStyledContent(styled), ResetColor, Print("\n"))?;
    Ok(())
}
