use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Not in a Godot project directory
    NotInProject,
    /// Config file not found
    ConfigNotFound,
    /// Invalid URL format
    InvalidUrl(String),
    /// Addon not found in repository
    AddonNotFound(String),
    /// Git operation failed
    GitError(String),
    /// IO error
    Io(std::io::Error),
    /// Other errors
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotInProject => write!(f, "Not in a Godot project directory"),
            Error::ConfigNotFound => write!(f, "gdam.yaml not found. Run 'gdam init' first"),
            Error::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            Error::AddonNotFound(msg) => write!(f, "Addon not found: {}", msg),
            Error::GitError(msg) => write!(f, "Git error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
