use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct Context {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl Context {
    pub fn insert<T: Any + Send + Sync + 'static>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }
}
