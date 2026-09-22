//! ## Flow
pub mod compiler;
pub mod context;
pub mod error;
pub mod instruction;
pub mod instructions;
pub mod procedure;
pub mod slots;
pub mod variable;


/*

* Whenever an instruction is called, it is passed MutexGuard<'_, SlotValue>.
* As a result it's upto instruction register to define what it does with the guard (macro will either Clone or use as ref)
* Slots are determined by the number of instructions in procedure. (SlotAddress determined by line number)

*/


pub fn compile() {}

pub async fn execute() {}

#[cfg(test)]
mod tests {}
