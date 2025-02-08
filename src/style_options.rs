use crossterm::style::Color;

#[derive(Debug, Clone)]
pub struct StyleOptions {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

/// Returns a new `StyleOptions` instance with no foreground or background colors set.
///
/// # Returns
///
/// A `StyleOptions` instance with both `foreground` and `background` set to `None`.
impl Default for StyleOptions {
    fn default() -> Self {
        StyleOptions {
            foreground: None,
            background: None,
        }
    }
}
