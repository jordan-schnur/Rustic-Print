//! # Rustic Print v2
//!
//! Rustic Print is a simple library for styling terminal output. It helps you render styled text blocks, interactive prompts, and tables.
//!
//! ## Message Display
//!
//! Most functions accept either a single string or a vector of strings, making it easy to print one-liners or multi-line messages.
//!
//! **Single String Example:**
//! ```rust
//! use rustic_print::RusticPrint;
//!
//! let printer = RusticPrint::new();
//! printer.success("Operation completed successfully!");
//! ```
//!
//! **Multiple Lines Example:**
//! ```rust
//! use rustic_print::RusticPrint;
//!
//! let printer = RusticPrint::new();
//! printer.info(vec![
//!     "Step 1: Initialization complete.",
//!     "Step 2: Processing data.",
//!     "Step 3: Operation finished.",
//! ]);
//! ```
//!
//! ## Tables
//!
//! Easily render tables by providing a vector of header strings and a vector of rows (each row is a vector of string slices).
//!
//! **Example:**
//! ```rust
//! use rustic_print::RusticPrint;
//!
//! let printer = RusticPrint::new();
//! let headers = vec!["Name", "Age", "Occupation"];
//! let rows = vec![
//!     vec!["Alice", "30", "Engineer"],
//!     vec!["Bob", "25", "Designer"],
//!     vec!["Charlie", "35", "Manager"],
//! ];
//! printer.table(headers, rows);
//! ```
//!
//! ## Confirmations
//!
//! Use the `confirm` function to prompt the user with a yes/no question. The default answer is provided as a boolean.
//!
//! **Example:**
//! ```rust
//! use rustic_print::RusticPrint;
//!
//! let printer = RusticPrint::new();
//! let proceed = printer.confirm("Do you want to continue?", true);
//!
//! if proceed {
//!     println!("Continuing...");
//! } else {
//!     println!("Operation cancelled.");
//! }
//! ```
//!
//! ## Interactive Choices
//!
//! The `choice` function displays a list of options and lets the user pick one interactively.
//!
//! **Example:**
//! ```rust
//! use rustic_print::RusticPrint;
//!
//! let printer = RusticPrint::new();
//! let options = ["Option 1", "Option 2", "Option 3"];
//! let selected = printer.choice("Select an option", &options, Some("Option 2"));
//!
//! println!("You selected: {}", selected);
//! ```
//!
//! ## Input with Validation
//!
//! The `ask` function not only prompts for input but can also enforce validation via a provided closure. If the input fails
//! validation, the prompt is repeated until a valid response is entered.
//!
//! **Example:**
//! ```rust
//! use rustic_print::RusticPrint;
//!
//! let printer = RusticPrint::new();
//!
//! // The validator requires the input to be at least 3 characters long.
//! let username = printer.ask(
//!     "Enter your username",
//!     Some("default_user"),
//!     Some(Box::new(|input| {
//!         if input.trim().len() >= 3 {
//!             Ok(())
//!         } else {
//!             Err("Username must be at least 3 characters long.".to_string())
//!         }
//!     }))
//! );
//!
//! println!("Your username is: {}", username);
//! ```
//! ## Available Functions
//!
//! The following functions are available on the [`RusticPrint`] struct. Click any item for more details:
//!
//! - [`RusticPrint::new`] - Create a new `RusticPrint` instance.
//! - [`RusticPrint::block`] - Print a styled text block.
//! - [`RusticPrint::underline_with_char`] - Underline a message with a repeated character.
//! - [`RusticPrint::title`] - Display a title with a styled underline.
//! - [`RusticPrint::section`] - Display a section header.
//! - [`RusticPrint::success`] - Print a success block with custom styling.
//! - [`RusticPrint::caution`] - Print a caution block with custom styling.
//! - [`RusticPrint::error`] - Print an error block with custom styling.
//! - [`RusticPrint::comment`] - Print a comment block prefixed with `//`.
//! - [`RusticPrint::warning`] - Print a warning block.
//! - [`RusticPrint::info`] - Print an informational block.
//! - [`RusticPrint::note`] - Print a note block with a custom prefix.
//! - [`RusticPrint::listing`] - Display a list of items.
//! - [`RusticPrint::text`] - Print wrapped text.
//! - [`RusticPrint::table`] - Render a table with headers and rows.
//! - [`RusticPrint::confirm`] - Prompt for a yes/no confirmation.
//! - [`RusticPrint::ask`] - Prompt for input with optional validation.
//! - [`RusticPrint::choice`] - Present an interactive choice prompt.
//!
//! For more details on each function, please refer to the full [API Reference](https://docs.rs/rustic_print/latest/rustic_print/).


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
use std::error::Error;
use std::fmt::Display;
use std::io::{self, stdout, Read, Write};
use std::time::Duration;
use crossterm::event::KeyModifiers;
use crossterm::terminal::{Clear, ClearType};
use textwrap::{fill, wrap, Options};

pub struct RusticPrint {}


impl RusticPrint {
    /// Creates a new instance of `RusticPrint`.
    ///
    /// # Returns
    ///
    /// A new `RusticPrint` instance.
    pub fn new() -> RusticPrint {
        RusticPrint {}
    }

    /// Prints a block of text using the provided messages and block options.
    ///
    /// This function converts the input into `Messages` and delegates rendering to the internal
    /// `render_block` function. It uses default block options if none are specified.
    ///
    /// # Arguments
    ///
    /// * `messages` - The message(s) to be printed; can be a single string or multiple strings.
    /// * `block_options` - Customization options for the block, including styling and prefix.
    ///
    /// # Panics
    ///
    /// Panics if rendering the block fails.
    pub fn block<T>(&self, messages: T, block_options: BlockOptions) -> ()
    where
        T: Into<Messages>,
    {
        self.render_block(messages, block_options)
            .expect("Failed to render b");
    }

    /// Renders a text block with the specified messages and block options.
    ///
    /// This is an internal helper function that:
    /// - Determines terminal width and wrapping.
    /// - Handles padding and styling.
    /// - Wraps the text appropriately.
    ///
    /// # Arguments
    ///
    /// * `message` - The message content, convertible into `Messages`.
    /// * `block_options` - Options that define the block's styling, prefix, and padding.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure in rendering the block.
    fn render_block<T>(&self, message: T, block_options: BlockOptions) -> Result<(), Box<dyn Error>>
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

    /// Renders a message with an underline composed of a repeated character.
    ///
    /// The function applies optional styling to both the message and its underline.
    ///
    /// # Arguments
    ///
    /// * `message` - The text to be underlined.
    /// * `underline_char` - The character used to create the underline.
    /// * `style_options` - Optional styling to apply to the text and underline.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or any encountered error.
    fn render_underline_with_char(
        &self,
        message: &str,
        underline_char: char,
        style_options: Option<StyleOptions>,
    ) -> Result<(), Box<dyn Error>> {
        let mut stdout = stdout();

        let mut message = style(message);
        let mut underline = style(underline_char.to_string().repeat(message.to_string().len()));

        if let Some(style_options) = style_options {
            if let Some(foreground) = style_options.foreground {
                message = message.with(foreground);
                underline = underline.with(foreground);
                queue!(stdout, SetForegroundColor(foreground))?;
            }
            if let Some(background) = style_options.background {
                message = message.on(background);
                underline = underline.on(background);
                queue!(stdout, SetBackgroundColor(background))?;
            }
        }

        queue!(
            stdout,
            PrintStyledContent(message),
            ResetColor,
            Print("\n"),
            PrintStyledContent(underline),
            ResetColor,
            Print("\n")
        )?;

        queue!(stdout, Print("\n"))?;
        stdout.flush()?;

        Ok(())
    }

    /// Underlines the given message with a repeated character.
    ///
    /// This function wraps around `render_underline_with_char` and panics if rendering fails.
    ///
    /// # Arguments
    ///
    /// * `message` - The text to underline.
    /// * `underline_char` - The character used to form the underline.
    /// * `style_options` - Optional styling parameters.
    pub fn underline_with_char(
        &self,
        message: &str,
        underline_char: char,
        style_options: Option<StyleOptions>,
    ) {
        self.render_underline_with_char(message, underline_char, style_options)
            .expect("Failed to render underline");
    }

    /// Displays a title by underlining the provided message with '=' characters.
    ///
    /// The title is styled with a dark green foreground.
    ///
    /// # Arguments
    ///
    /// * `message` - The title text.
    pub fn title(&self, message: &str) {
        self.underline_with_char(
            message,
            '=',
            Some(StyleOptions {
                foreground: Some(Color::DarkGreen),
                background: None,
            }),
        );
    }

    /// Displays a section header by underlining the message with '-' characters.
    ///
    /// The section header is styled with a dark green foreground.
    ///
    /// # Arguments
    ///
    /// * `message` - The section header text.
    pub fn section(&self, message: &str) {
        self.underline_with_char(
            message,
            '-',
            Some(StyleOptions {
                foreground: Some(Color::DarkGreen),
                background: None,
            }),
        );
    }

    /// Prints a success block with black text on a green background.
    ///
    /// The block is labeled with "OK" and includes padding.
    ///
    /// # Arguments
    ///
    /// * `messages` - The message(s) to display, convertible into `Messages`.
    ///
    /// # Panics
    ///
    /// Panics if rendering the success block fails.
    pub fn success<T>(&self, messages: T)
    where
        T: Into<Messages>
    {
        self.render_block(
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
        )
        .expect("Failed to print success block");
    }

    /// Prints a caution block with grey text on a dark red background.
    ///
    /// The block is labeled "CAUTION" and uses a custom prefix along with padding.
    ///
    /// # Arguments
    ///
    /// * `messages` - The message(s) to display, convertible into `Messages`.
    ///
    /// # Panics
    ///
    /// Panics if rendering the caution block fails.
    pub fn caution<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.render_block(
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

    /// Prints an error block with grey text on a dark red background.
    ///
    /// The block is labeled "ERROR" and includes padding.
    ///
    /// # Arguments
    ///
    /// * `messages` - The error message(s) to display, convertible into `Messages`.
    ///
    /// # Panics
    ///
    /// Panics if rendering the error block fails.
    pub fn error<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.render_block(
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

    /// Prints a comment block prefixed with "//".
    ///
    /// Useful for displaying comments or annotations.
    ///
    /// # Arguments
    ///
    /// * `messages` - The comment message(s), convertible into `Messages`.
    ///
    /// # Panics
    ///
    /// Panics if rendering the comment block fails.
    pub fn comment<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.render_block(
            messages,
            BlockOptions {
                prefix: " // ".to_string(),
                ..Default::default()
            },
        )
        .expect("Failed to print comment block");
    }

    /// Prints a warning block with black text on a dark yellow background.
    ///
    /// The block is labeled "WARNING" and includes padding.
    ///
    /// # Arguments
    ///
    /// * `messages` - The warning message(s) to display, convertible into `Messages`.
    ///
    /// # Panics
    ///
    /// Panics if rendering the warning block fails.
    pub fn warning<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.render_block(
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

    /// Prints an informational block with green text.
    ///
    /// The block is labeled "INFO" and includes padding.
    ///
    /// # Arguments
    ///
    /// * `messages` - The informational message(s) to display, convertible into `Messages`.
    ///
    /// # Panics
    ///
    /// Panics if rendering the info block fails.
    pub fn info<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.render_block(
            messages,
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

    /// Prints a note block with dark yellow text.
    ///
    /// The block is labeled "NOTE", uses a custom prefix, and does not include padding.
    ///
    /// # Arguments
    ///
    /// * `messages` - The note message(s) to display, convertible into `Messages`.
    ///
    /// # Panics
    ///
    /// Panics if rendering the note block fails
    pub fn note<T>(&self, messages: T)
    where
        T: Into<Messages>,
    {
        self.render_block(
            messages,
            BlockOptions {
                style: Some(StyleOptions {
                    foreground: Some(Color::DarkYellow),
                    background: None,
                }),
                block_type: Some("NOTE".to_string()),
                prefix: " ! ".to_string(),
                ..Default::default()
            },
        )
        .expect("Failed to print info block");
    }

    /// Displays a list of items, each preceded by an asterisk.
    ///
    /// # Arguments
    ///
    /// * `items` - A vector of items that implement the `Display` trait.
    ///
    /// # Panics
    ///
    /// Panics if printing any of the items fails.
    pub fn listing<T>(&self, items: Vec<T>)
    where
        T: std::fmt::Display,
    {
        let mut stdout = stdout();
        for (i, item) in items.iter().enumerate() {
            queue!(stdout, Print(format!("* {}\n", item))).expect("Failed to queue print");
        }

        stdout.flush().expect("Failed to flush stdout");
    }

    /// Prints text with automatic wrapping based on the terminal width.
    ///
    /// The text is indented with a single space on each line.
    ///
    /// # Arguments
    ///
    /// * `message` - The text to print, which implements the `Display` trait.
    ///
    /// # Panics
    ///
    /// Panics if printing the text fails.
    pub fn text<T>(&self, message: T)
    where
        T: std::fmt::Display,
    {
        let mut stdout = stdout();

        let term_width = terminal::size().unwrap_or((120, 0)).0 as usize;
        let options = Options::new(term_width.clone() - 1)
            .initial_indent(" ")
            .subsequent_indent(" ");

        for line in fill(&*message.to_string(), options).lines() {
            queue!(stdout, Print(line), Print("\n")).expect("Failed to queue print");
        }

        stdout.flush().expect("Failed to flush stdout");
    }

    /// Prints a table with the specified headers and rows.
    ///
    /// Internally, this function creates a `Table` instance and calls its `print_table` method.
    ///
    /// # Arguments
    ///
    /// * `headers` - A vector of string slices representing the table headers.
    /// * `rows` - A vector of rows, where each row is a vector of string slices.
    pub fn table(&self, headers: Vec<&str>, rows: Vec<Vec<&str>>) {
        let table = Table::new(headers, rows);
        table.print_table();
    }

    /// Prompts the user for confirmation with a yes/no question.
    ///
    /// The function enters raw mode, displays the question with default highlighting,
    /// and processes keyboard input until the user confirms with Enter.
    ///
    /// # Arguments
    ///
    /// * `question` - The question to present to the user.
    /// * `default` - The default answer if the user provides no input.
    ///
    /// # Returns
    ///
    /// Returns `true` if the user confirms (yes), otherwise `false`.
    pub fn confirm(&self, question: &str, default: bool) -> bool {
        let mut stdout = io::stdout();
        enable_raw_mode().expect("Failed to enable raw mode");

        let default_answer = if default { "yes" } else { "no" };

        // Using the Stylize trait to color the prompt.
        print!(
            "{} (yes/no) [{}]:\r\n > ",
            question.green(),
            default_answer.yellow()
        );
        stdout.flush().expect("Failed to flush stdout");

        let mut input = String::new();

        loop {
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
                            print!("\x08 \x08"); // Visual backspace.
                        }
                    }
                    _ => {}
                }
                stdout.flush().expect("Failed to flush stdout");
            }
        }

        disable_raw_mode().expect("Failed to disable raw mode");
        println!();

        if input.trim().is_empty() {
            default
        } else if input.trim().eq_ignore_ascii_case("yes")
            || input.trim().eq_ignore_ascii_case("y")
        {
            true
        } else {
            false
        }
    }

    /// Prompts the user with a question and returns the response.
    ///
    /// If a default value is provided and the user enters nothing, the default is used.
    /// An optional validator function can be supplied to validate the user's input.
    ///
    /// # Arguments
    ///
    /// * `question` - The question to display.
    /// * `default` - An optional default answer.
    /// * `validator` - An optional closure that validates the input and returns `Ok(())`
    ///   if valid, or an error message otherwise.
    ///
    /// # Returns
    ///
    /// Returns the user's input as a `String`.
    ///
    /// # Panics
    ///
    /// Panics if reading from stdin fails.
    pub fn ask(
        &self,
        question: &str,
        default: Option<&str>,
        validator: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
    ) -> String {
        let mut stdout = io::stdout();

        loop {
            Self::ask_question(question, default);
            stdout.flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();

            // Use the default value if the user provides no input.
            let answer = if input.is_empty() {
                default.unwrap_or("").to_string()
            } else {
                input.to_string()
            };

            // Validate the answer if a validator was provided.
            if let Some(ref validate) = validator {
                match validate(&answer) {
                    Ok(_) => return answer,
                    Err(err) => {
                        println!("{}", err.red());
                    }
                }
            } else {
                return answer;
            }
        }
    }

    /// Displays a prompt for the user, including an optional default value.
    ///
    /// This is a helper function for the `ask` method.
    ///
    /// # Arguments
    ///
    /// * `question` - The question to display.
    /// * `default_text` - An optional default text to show.
    fn ask_question(question: &str, default_text: Option<&str>) {
        let default_text = if let Some(dt) = default_text {
            format!(" [{}]", dt.dark_green())
        } else {
            String::new()
        };

        print!("{}{}:\n> ", question.dark_green(), default_text);
    }

    /// Presents a multiple-choice question to the user and returns the selected option.
    ///
    /// The function displays a list of choices, allows navigation via arrow keys, and handles
    /// input validation. It employs raw mode for interactive input.
    ///
    /// # Arguments
    ///
    /// * `question` - The prompt question.
    /// * `choices` - A slice of choices to select from.
    /// * `default` - An optional default choice.
    ///
    /// # Returns
    ///
    /// Returns the selected choice as a `String`.
    ///
    /// # Panics
    ///
    /// Panics if reading events or flushing output fails.
    pub fn choice(&self, question: &str, choices: &[&str], default: Option<&str>) -> String {
        loop {
            // Re-render the entire question block.
            let mut stdout = stdout();
            if default.is_some() {
                print!("{} [{}]:\n", question.green(), default.unwrap().green());
            } else {
                print!("{}:\n", question.green());
            }
            for (i, choice) in choices.iter().enumerate() {
                println!("  [{}] {}", i.to_string().green(), choice);
            }
            // Print the prompt line.
            print!("> ");
            stdout.flush().unwrap();
            // Save the current cursor row for the prompt.
            let (_, prompt_row) = crossterm::cursor::position().unwrap();

            // Enable raw mode for interactive input.
            enable_raw_mode().expect("Failed to enable raw mode");

            // Initialize the input buffer and selection.
            let mut input_buffer = String::new();
            // If the default text exactly matches one of the choices, use its index; otherwise, default to 0.
            let mut selected_index = choices
                .iter()
                .position(|&c| c == default.unwrap_or(""))
                .unwrap_or(0);

            // Immediately display the default selection.
            if default.is_some() {
                print!("{}", choices[selected_index]);
            }

            stdout.flush().unwrap();

            // Inner loop: process key events.
            loop {
                if event::poll(Duration::from_millis(500)).unwrap() {
                    match event::read().unwrap() {
                        Event::Key(key_event) => {
                            match key_event.code {
                                KeyCode::Enter => break,
                                // Tab completes the suggestion.
                                KeyCode::Tab => {
                                    input_buffer = choices[selected_index].to_string();
                                }
                                KeyCode::Up => {
                                    selected_index = if selected_index == 0 {
                                        choices.len() - 1
                                    } else {
                                        selected_index - 1
                                    };
                                    input_buffer.clear();
                                }
                                KeyCode::Down => {
                                    selected_index = (selected_index + 1) % choices.len();
                                    input_buffer.clear();
                                }
                                KeyCode::Char(c) => {
                                    if c == 'c' && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                                        disable_raw_mode().unwrap();
                                        std::process::exit(0);
                                    }

                                    input_buffer.push(c);

                                    // If the buffer parses as a valid index, update selection.
                                    if let Ok(idx) = input_buffer.parse::<usize>() {
                                        if idx < choices.len() {
                                            selected_index = idx;
                                        }
                                    }
                                    // Otherwise, if the text starts any choice (case-insensitive), update selection.
                                    for (i, &choice) in choices.iter().enumerate() {
                                        if choice.to_lowercase().starts_with(&input_buffer.to_lowercase()) {
                                            selected_index = i;
                                            break;
                                        }
                                    }
                                }
                                KeyCode::Backspace => {
                                    input_buffer.pop();
                                }
                                _ => {}
                            }
                            // Update the prompt line.
                            use crossterm::{
                                queue,
                                terminal::{Clear, ClearType},
                                cursor::MoveTo,
                                style::{SetForegroundColor, SetBackgroundColor, ResetColor, Color},
                            };
                            queue!(
                            stdout,
                            MoveTo(2, prompt_row),
                            Clear(ClearType::UntilNewLine)
                        )
                                .unwrap();

                            if input_buffer.is_empty() {
                                // If nothing has been typed, display the full default/suggestion in normal style.
                                print!("{}", choices[selected_index]);
                            } else {
                                let suggestion = choices[selected_index];
                                // If the suggestion begins with the user's input (case-insensitive)
                                if suggestion.to_lowercase().starts_with(&input_buffer.to_lowercase()) {
                                    let remainder = &suggestion[input_buffer.len()..];
                                    // Print the user's input as usual.
                                    print!("{}", input_buffer);
                                    // Print the remainder with grey background and white foreground.
                                    queue!(
                                    stdout,
                                    SetForegroundColor(Color::White),
                                    SetBackgroundColor(Color::Grey)
                                )
                                        .unwrap();
                                    print!("{}", remainder);
                                    // Reset styling.
                                    queue!(stdout, ResetColor).unwrap();
                                } else {
                                    print!("{}", input_buffer);
                                }
                            }
                            stdout.flush().unwrap();
                        }
                        _ => {}
                    }
                }
            } // end inner input loop

            // Disable raw mode.
            disable_raw_mode().expect("Failed to disable raw mode");
            println!();

            // Determine the final selection.
            let final_choice = if input_buffer.is_empty() {
                choices[selected_index]
            } else if let Ok(idx) = input_buffer.parse::<usize>() {
                if idx < choices.len() {
                    choices[idx]
                } else {
                    ""
                }
            } else {
                let mut found = "";
                for &choice in choices {
                    if choice.to_lowercase() == input_buffer.to_lowercase() {
                        found = choice;
                        break;
                    }
                }
                found
            };

            // If the selection is invalid, render an error block and restart.
            if final_choice.is_empty() {
                self.error(format!(
                    "Invalid selection: \"{}\". Please enter a valid index or choice.",
                    input_buffer
                ));
                continue;
            } else if selected_index > choices.len() {
                self.error(format!(
                    "Invalid selection: \"{}\". Please enter a valid index or choice.",
                    input_buffer
                ));
                continue;
            } else {
                return final_choice.to_string();
            }
        }
    }
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
) -> Result<(), Box<dyn Error>> {
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
) -> Result<(), Box<dyn Error>> {
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

/// Helper function for the `choice` method: prints the choices with the current selection highlighted.
///
/// For the selected option, the text is rendered with inverted colors.
fn print_choices(choices: &[&str], selected: usize) {
    // For simplicity, reprint all choices.
    // The selected option is rendered with inverted colors.
    for (i, choice) in choices.iter().enumerate() {
        if i == selected {
            // For the selected option, use a black-on-white style.
            println!("{}", format!("> {}", choice).black().on_white());
        } else {
            println!("  {}", choice);
        }
    }
}