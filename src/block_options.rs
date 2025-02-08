use crate::messages::Messages;
use crate::style_options::StyleOptions;

#[derive(Debug, Clone)]
pub struct BlockOptions {
    pub block_type: Option<String>,
    pub style: Option<StyleOptions>,
    pub prefix: String,
    pub padding: bool,
}

/// Returns a default instance of `BlockOptions`.
///
/// # Returns
///
/// A `BlockOptions` instance with:
/// - `block_type`: `None`
/// - `style`: `None`
/// - `prefix`: an empty string
/// - `padding`: `false`
impl Default for BlockOptions {
    fn default() -> Self {
        BlockOptions {
            block_type: None,
            style: None,
            prefix: "".to_string(),
            padding: false,
        }
    }
}
