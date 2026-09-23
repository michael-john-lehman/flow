use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
/// **Context**
/// 
/// Provides immutable type map that is provided to each instruction.
/// 
/// This allows access to outside resources that cannot be accessed via some static cell.
/// 
/// For example, HTTP Request body.
/// 
pub struct Context(HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>);

impl Context {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }
    
    pub fn insert<T: Any + Send + Sync + 'static>(&mut self, value: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        self.0.get(&TypeId::of::<T>()).and_then(|boxed| boxed.downcast_ref())
    }
}
