pub mod instruction;
pub use instruction::Instruction;

use crate::error::RuntimeError;

inventory::collect!(Instruction);

pub fn iter() -> impl Iterator<Item = &'static Instruction> {
    inventory::iter::<Instruction>()
}

// partial example;
//  Key thing to notice is that we are able to drop MutexGuard<> earlier
//  Since the borrow checker is merely enforcing that where we borrow from lasts at least as long as 
#[allow(unused)]
static A: Instruction = Instruction {
    module: "",
    name: "",
    parameters: &[(std::any::TypeId::of::<i32>(), "i32")],
    outputs: &[(std::any::TypeId::of::<i32>(), "i32")],
    f: |context, params| {
        Box::pin(async move {
            let i = params[0].as_ref().and_then(|item| item.downcast_ref::<i32>()).ok_or(RuntimeError::invalid_address())?;
            for i in params {
                drop(i);
            }
            Ok((1, Box::new(0) as Box<dyn std::any::Any + Send + Sync + 'static>))
        })
    }
};
