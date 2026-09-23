use crate::error::RuntimeError;
use crate::mem::Context;
use futures_util::future::BoxFuture;
use std::any::{Any, TypeId};
use tokio::sync::MutexGuard;

/// Basic idea is that `(Context, Params) -> Result<(Branch, Variable), RuntimeError>`
pub type InstructionFunction = for<'f> fn(&'f Context, Vec<MutexGuard<'f, Option<Box<dyn Any + Send + Sync + 'static>>>>) -> BoxFuture<'f, Result<(usize, Box<dyn Any + Send + Sync + 'static>), RuntimeError>>;

#[derive(Clone, Copy)]
pub struct Instruction {
    /// The module where our function/operation is defined
    pub module: &'static str,
    /// The name of our function/operation
    pub name: &'static str,
    /// The parameters to our operation/function. Which item is the expected (TypeId, TypeName).
    pub parameters: &'static [(TypeId, &'static str)],
    /// The output branches and their corresponding branches (TypeId, TypeName)
    pub outputs: &'static [(TypeId, &'static str)],
    /// The actual function to call
    pub f: InstructionFunction
}
