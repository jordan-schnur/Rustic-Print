#[derive(Clone, Debug)]
pub(crate) enum Messages {
    Single(String),
    Multiple(Vec<String>),
}

impl From<String> for Messages {
    fn from(s: String) -> Self {
        Messages::Single(s)
    }
}

impl From<&str> for Messages {
    fn from(s: &str) -> Self {
        Messages::Single(s.to_string())
    }
}

impl From<Vec<String>> for Messages {
    fn from(vec: Vec<String>) -> Self {
        Messages::Multiple(vec)
    }
}

impl From<Vec<&str>> for Messages {
    fn from(vec: Vec<&str>) -> Self {
        Messages::Multiple(vec.into_iter().map(String::from).collect())
    }
}
