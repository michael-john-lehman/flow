use crate::instruction::Instruction;

inventory::collect!(Instruction);

pub fn iter() -> impl Iterator<Item = &'static Instruction> {
    inventory::iter::<Instruction>()
}
