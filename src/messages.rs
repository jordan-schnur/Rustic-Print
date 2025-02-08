#[derive(Clone, Debug)]
pub enum Messages {
    Single(String),
    Multiple(Vec<String>),
}

/// Converts a `String` into a `Messages::Single` variant.
///
/// # Arguments
///
/// * `s` - A string to be wrapped as a single message.
impl From<String> for Messages {
    fn from(s: String) -> Self {
        Messages::Single(s)
    }
}

/// Converts a string slice into a `Messages::Single` variant.
///
/// # Arguments
///
/// * `s` - A string slice to be converted and wrapped as a single message.
impl From<&str> for Messages {
    fn from(s: &str) -> Self {
        Messages::Single(s.to_string())
    }
}

/// Converts a vector of `String` into a `Messages::Multiple` variant.
///
/// # Arguments
///
/// * `vec` - A vector of strings.
impl From<Vec<String>> for Messages {
    fn from(vec: Vec<String>) -> Self {
        Messages::Multiple(vec)
    }
}

/// Converts a vector of string slices into a `Messages::Multiple` variant.
///
/// # Arguments
///
/// * `vec` - A vector of string slices.
impl From<Vec<&str>> for Messages {
    fn from(vec: Vec<&str>) -> Self {
        Messages::Multiple(vec.into_iter().map(String::from).collect())
    }
}
