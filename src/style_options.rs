use crossterm::style::Color;

#[derive(Debug, Clone)]
pub struct StyleOptions {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl Default for StyleOptions {
    fn default() -> Self {
        StyleOptions {
            foreground: None,
            background: None,
        }
    }
}