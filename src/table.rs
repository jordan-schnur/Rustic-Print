use std::fmt::Display;
use crossterm::style::{style, Stylize};

pub struct Table<'a> {
    headers: Vec<&'a str>,
    rows: Vec<Vec<&'a str>>,
    column_widths: Vec<usize>,
}

impl<'a> Table<'a> {
    /// Creates a new `Table` instance with the given headers and rows.
    ///
    /// It calculates the column widths based on the maximum length of header and cell contents.
    ///
    /// # Arguments
    ///
    /// * `headers` - A vector of string slices representing the table headers.
    /// * `rows` - A vector of rows, where each row is a vector of string slices.
    ///
    /// # Returns
    ///
    /// A new instance of `Table`.
    pub fn new(headers: Vec<&'a str>, rows: Vec<Vec<&'a str>>) -> Table<'a> {
        // Initialize each column width to the header length.
        let mut column_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

        // Update widths based on row cell lengths.
        for row in &rows {
            for (i, cell) in row.iter().enumerate() {
                if cell.len() > column_widths[i] {
                    column_widths[i] = cell.len();
                }
            }
        }

        Table {
            headers,
            rows,
            column_widths,
        }
    }

    /// Creates a horizontal line based on the widths of each column.
    ///
    /// # Arguments
    ///
    /// * `filler` - The character to repeat for the line.
    ///
    /// # Returns
    ///
    /// A `String` representing the horizontal line.
    pub fn create_line(&self, filler: char) -> String {
        self.column_widths
            .iter()
            .map(|&len| filler.to_string().repeat(len + 2))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Prints the table to stdout with styled headers and borders.
    pub fn print_table(&self) {
        let border_line = self.create_line('-');
        println!("{}", border_line);

        // Print headers in green and bold.
        let header_str = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, header)| {
                format!(
                    " {}{} ",
                    header.green().bold(),
                    " ".repeat(self.column_widths[i].saturating_sub(header.len()))
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", header_str);

        println!("{}", border_line);

        // Print rows.
        for row in &self.rows {
            let row_str = row
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    format!(
                        " {}{} ",
                        cell,
                        " ".repeat(self.column_widths[i].saturating_sub(cell.len()))
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", row_str);
        }

        println!("{}", border_line);
    }
}