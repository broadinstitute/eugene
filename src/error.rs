use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Error {
    message: String,
    source: Option<Box<Error>>,
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::from(message.to_string())
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        let source = None;
        Error { message, source }
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self {
        let source = Error::from(reqwest_error.to_string());
        source.prepend_str("Reqwest")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            None => { write!(f, "{}", self.message) }
            Some(source) => { write!(f, "{}: {}", self.message, source) }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            None => { None }
            Some(child) => { Some(child) }
        }
    }
}

impl Error {
    pub fn prepend_str(self, message: &str) -> Error {
        self.prepend(message.to_string())
    }
    pub fn prepend(self, message: String) -> Error {
        let source = Some(Box::new(self));
        Error { message, source }
    }
}