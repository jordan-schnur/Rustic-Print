use crossterm::style::{Color, Colors};
use crate::style_options::StyleOptions;

pub struct BlockOptions {
    pub(crate) name: Option<String>,
    pub(crate) style: Option<StyleOptions>,
    pub(crate) prefix: Option<String>,
    pub(crate) padding: bool,
    pub(crate) line_width: usize,
    pub(crate) escape: bool,
}

impl Default for BlockOptions {
    fn default() -> Self {
        BlockOptions {
            name: None,
            style: None,
            prefix: None,
            padding: false,
            line_width: 120,
            escape: true,
        }
    }
}