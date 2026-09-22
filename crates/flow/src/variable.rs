use std::any::*;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct TypeSignature {
    pub id: TypeId,
    pub name: &'static str,
}

impl TypeSignature {
    pub fn of<T: Any>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            name: type_name::<T>(),
        }
    }
}

pub trait Variable: Any + Debug + Send + Sync + 'static {
    fn type_signature(&self) -> TypeSignature;
}

impl<T> Variable for T
where
    T: Any + Debug + Send + Sync + 'static
{
    fn type_signature(&self) -> TypeSignature {
        TypeSignature::of::<T>()
    }
}

impl dyn Variable {

    pub fn downcast_ref<T: Variable>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Variable>(&mut self) -> Option<&mut T> {
        (self as &mut dyn Any).downcast_mut::<T>()
    }

}
