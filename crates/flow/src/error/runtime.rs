use std::any::TypeId;
use std::error::Error;
use std::fmt::{Debug, Display};

pub enum RuntimeError {
    /// Failed to downcast
    Downcast {
        expected: (&'static str, TypeId),
        actual: (&'static str, TypeId),
    },
    /// Something that should not happen, happened
    Unexpected {
        detail: &'static str,
    },
    /// Attempted to access address outside bound
    InvalidSlotAdddress,
}

impl RuntimeError {
    pub const fn downcast(expected: (&'static str, TypeId), actual: (&'static str, TypeId)) -> Self {
        Self::Downcast { expected, actual }
    }

    pub const fn unexpected(detail: &'static str) -> Self {
        Self::Unexpected { detail }
    }

    pub const fn invalid_slot_address() -> Self {
        Self::InvalidSlotAdddress
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
