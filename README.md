[![Rust](https://github.com/jordan-schnur/Rustic-Print/actions/workflows/.rust.yml/badge.svg)](https://github.com/jordan-schnur/Rustic-Print/actions/workflows/.rust.yml)
# Rustic Print

Rustic Print is a lightweight, versatile Rust library for enhancing terminal output. It simplifies the process of rendering styled text blocks, interactive prompts, and tables—ideal for building engaging and informative command-line interfaces.

[API Reference](https://docs.rs/rustic_print/latest/rustic_print/)

## Features

- **Styled Text Blocks**: Create customizable blocks with color, padding, and custom prefixes.
- **Message Types**: Easily display success, error, warning, caution, info, comment, note, and more.
- **Tables**: Generate tables with auto-calculated column widths for neat, aligned output.
- **Interactive Prompts**: Built-in functions for confirmations, input with validation, and interactive choice selection.
- **Automatic Text Wrapping**: Dynamically wraps text to fit terminal width with proper indentations.
- **Flexible API**: Accepts both single strings and vectors of strings to handle one-liners or multi-line messages.

## Installation

Add Rustic Print to your project's `Cargo.toml`:

```toml
[dependencies]
rustic_print = "0.2.0"
```

## Usage

### Message Display

Most functions accept either a single string or a vector of strings, making it easy to print one-liners or multi-line messages.

**Single String Example:**
```rust
use rustic_print::RusticPrint;

let printer = RusticPrint::new();
printer.success("Operation completed successfully!");
```

**Multiple Lines Example:**
```rust
use rustic_print::RusticPrint;

let printer = RusticPrint::new();
printer.info(vec![
    "Step 1: Initialization complete.",
    "Step 2: Processing data.",
    "Step 3: Operation finished.",
]);
```

### Tables

Easily render tables by providing a vector of header strings and a vector of rows (each row is a vector of string slices).

**Example:**
```rust
use rustic_print::RusticPrint;

let printer = RusticPrint::new();
let headers = vec!["Name", "Age", "Occupation"];
let rows = vec![
    vec!["Alice", "30", "Engineer"],
    vec!["Bob", "25", "Designer"],
    vec!["Charlie", "35", "Manager"],
];
printer.table(headers, rows);
```

### Confirmations

Use the `confirm` function to prompt the user with a yes/no question. The default answer is provided as a boolean.

**Example:**
```rust
use rustic_print::RusticPrint;

let printer = RusticPrint::new();
let proceed = printer.confirm("Do you want to continue?", true);

if proceed {
    println!("Continuing...");
} else {
    println!("Operation cancelled.");
}
```

### Interactive Choices

The `choice` function displays a list of options and lets the user pick one interactively.

**Example:**
```rust
use rustic_print::RusticPrint;

let printer = RusticPrint::new();
let options = ["Option 1", "Option 2", "Option 3"];
let selected = printer.choice("Select an option", &options, Some("Option 2"));

println!("You selected: {}", selected);
```

### Input with Validation

The `ask` function not only prompts for input but can also enforce validation via a provided closure. If the input fails validation, the prompt is repeated until a valid response is entered.

**Example:**
```rust
use rustic_print::RusticPrint;

let printer = RusticPrint::new();

// The validator requires the input to be at least 3 characters long.
let username = printer.ask(
    "Enter your username",
    Some("default_user"),
    Some(Box::new(|input| {
        if input.trim().len() >= 3 {
            Ok(())
        } else {
            Err("Username must be at least 3 characters long.".to_string())
        }
    }))
);

println!("Your username is: {}", username);
```

## Available Functions

The following functions are available on the `RusticPrint` struct:

- `RusticPrint::new` - Create a new `RusticPrint` instance.
- `RusticPrint::block` - Print a styled text block.
- `RusticPrint::underline_with_char` - Underline a message with a repeated character.
- `RusticPrint::title` - Display a title with a styled underline.
- `RusticPrint::section` - Display a section header.
- `RusticPrint::success` - Print a success block with custom styling.
- `RusticPrint::caution` - Print a caution block with custom styling.
- `RusticPrint::error` - Print an error block with custom styling.
- `RusticPrint::comment` - Print a comment block prefixed with `//`.
- `RusticPrint::warning` - Print a warning block.
- `RusticPrint::info` - Print an informational block.
- `RusticPrint::note` - Print a note block with a custom prefix.
- `RusticPrint::listing` - Display a list of items.
- `RusticPrint::text` - Print wrapped text.
- `RusticPrint::table` - Render a table with headers and rows.
- `RusticPrint::confirm` - Prompt for a yes/no confirmation.
- `RusticPrint::ask` - Prompt for input with optional validation.
- `RusticPrint::choice` - Present an interactive choice prompt.

For more details on each function, please refer to the full [API Reference](https://docs.rs/rustic_print/latest/rustic_print/).

