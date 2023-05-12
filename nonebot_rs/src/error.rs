#[derive(Debug, Clone)]
pub enum NBError {
    Text(String),
    Other(crate::message::MessageChain),
    State(crate::matcher::Session),
}

impl std::error::Error for NBError {}

impl std::fmt::Display for NBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self)
    }
}

pub type NBResult<T> = Result<T, NBError>;

impl From<&str> for NBError {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<String> for NBError {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<crate::message::MessageChain> for NBError {
    fn from(value: crate::message::MessageChain) -> Self {
        Self::Other(value)
    }
}

