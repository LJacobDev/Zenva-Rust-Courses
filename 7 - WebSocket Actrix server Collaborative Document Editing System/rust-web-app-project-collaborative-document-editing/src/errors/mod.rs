use std::fmt::{Display, Formatter};

/// Represents potential errors that can occur during ws connection creation process
pub enum ConnectionError {
    // indicates an error occurred while creating ws session
    CreateServerError(String), // stores a description of the server creation error
}

impl Display for ConnectionError {

    // Formats the ConnectionError for display as a human-readable string
    fn fmt(&self, f: &mut formatter<'_>) -> std::fmt::Result {
        match *self {
            ConnectionError::CreateServerError(ref desc) => {
                write!(f, "Error while creating server. {}", desc)
            }
        }
    }
}
