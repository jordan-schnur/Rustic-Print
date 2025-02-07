#[derive(Clone, Debug)]
pub(crate) enum Messages {
    Single(String),
    Multiple(Vec<String>),
}