use crate::error::*;
use std::any::Any;

pub struct Slots(Vec<Option<Box<dyn Any + Send + Sync + 'static>>>);

impl Slots {
    pub fn initalize(size: usize) -> Self {
        Self((0..size).map(|_| None).collect())
    }

    pub fn get<T: Any + Send + Sync + 'static>(&self, position: usize) -> Result<&T, RuntimeError> {
        self.0.get(position)
            .ok_or(RuntimeError::invalid_slot())?
            .as_ref()
            .ok_or(RuntimeError::null())?
            .downcast_ref::<T>()
            .ok_or(RuntimeError::downcast())
    }

    pub fn put<T: Any + Send + Sync + 'static>(&mut self, value: T, position: usize) -> Result<(), RuntimeError> {
        let ptr = self.0.get_mut(position).ok_or(RuntimeError::invalid_slot())?;
        *ptr = Some(Box::new(value));
        Ok(())
    }
}
