[![Rust](https://github.com/jordan-schnur/Rustic-Print/actions/workflows/.rust.yml/badge.svg)](https://github.com/jordan-schnur/Rustic-Print/actions/workflows/.rust.yml)
# Rustic Print

Rustic Print is a lightweight, versatile Rust library for enhancing terminal output. It simplifies the process of rendering styled text blocks, interactive prompts, and tables—ideal for building engaging and informative command‐line interfaces.

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