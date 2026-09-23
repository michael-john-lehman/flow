use std::error::Error;
use std::fmt::{Debug, Display};

pub enum RuntimeError {
    /// Attempted to access invalid slot address
    InvalidAddress
}

impl RuntimeError {
    pub const fn invalid_address() -> Self {
        Self::InvalidAddress
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for RuntimeError {}
