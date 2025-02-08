use crate::messages::Messages;
use crate::style_options::StyleOptions;

#[derive(Debug, Clone)]
pub struct BlockOptions {
    pub block_type: Option<String>,
    pub style: Option<StyleOptions>,
    pub prefix: String,
    pub padding: bool,
}

impl Default for BlockOptions {
    fn default() -> Self {
        BlockOptions {
            block_type: None,
            style: None,
            prefix: "".to_string(),
            padding: false
        }
    }
}