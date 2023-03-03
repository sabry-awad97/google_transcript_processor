use std::error::Error;

#[derive(Debug)]
pub enum MergeTranscriptError {
    IoError(std::io::Error),
    InvalidInput(String),
}

impl From<std::io::Error> for MergeTranscriptError {
    fn from(error: std::io::Error) -> Self {
        MergeTranscriptError::IoError(error)
    }
}

impl std::fmt::Display for MergeTranscriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MergeTranscriptError::IoError(e) => write!(f, "I/O error: {}", e),
            MergeTranscriptError::InvalidInput(s) => write!(f, "Invalid input: {}", s),
        }
    }
}

impl Error for MergeTranscriptError {}
