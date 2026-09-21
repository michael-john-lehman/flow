use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug)]
pub enum CompilationErrorKind {

}

#[derive(Clone, Copy, Debug)]
pub enum RuntimeErrorKind {
    Null,
    Downcast,
    InvalidSlot,
}

pub struct CompilationError {
    pub kind: CompilationErrorKind,
}

pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    /// If error occurs during registered function, the returned error will be moved here.
    pub err: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl CompilationError {}

impl RuntimeError {
    pub const fn null() -> Self {
        Self {
            kind: RuntimeErrorKind::Null,
            err: None,
        }
    }

    pub const fn invalid_slot() -> Self {
        Self {
            kind: RuntimeErrorKind::InvalidSlot,
            err: None
        }
    }

    pub const fn downcast() -> Self {
        Self {
            kind: RuntimeErrorKind::Downcast,
            err: None
        }
    }
}

impl Debug for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for CompilationError {}

impl Error for RuntimeError {}
