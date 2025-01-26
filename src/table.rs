use crate::CLEAR_COLOR;
use crate::console_color::ConsoleColor;

pub struct Table<'a> {
    headers: Vec<&'a str>,
    rows: Vec<Vec<&'a str>>,
    column_widths: Vec<usize>,
}

impl<'a> Table<'a> {
    pub fn new(headers: Vec<&'a str>, rows: Vec<Vec<&'a str>>) -> Table<'a> {
        let mut column_widths: Vec<usize> = vec![0; headers.len()];

        for (index, header) in headers.iter().enumerate() {
            let header_len = header.len();
            if header.len() > column_widths[index] {
                column_widths[index] = header.len();
            }

            for (j, cell) in rows[index].iter().enumerate() {
                if cell.len() > column_widths[index] {
                    column_widths[index] = cell.len();
                }
            }
        }

        Table {
            headers,
            rows,
            column_widths,
        }
    }

    pub fn create_line(&self, filler: char) -> String {
        self.column_widths
            .iter()
            .map(|&len| filler.to_string().repeat(len + 2))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn print_table(&self) {
        let border_line = self.create_line('-');
        println!("{}", border_line);

        // Print headers
        let header_str = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, header)| {
                format!(
                    " {}{}{} ",
                    ConsoleColor::Green.to_fg_ansi_code(),
                    header,
                    " ".repeat(self.column_widths[i].saturating_sub(header.len()))
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
            + CLEAR_COLOR;
        println!("{}", header_str);


        println!("{}", border_line);

        // Print rows
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
                .join(" ")
                + CLEAR_COLOR;
            println!("{}", row_str);
        }

        println!("{}", border_line);
    }
}