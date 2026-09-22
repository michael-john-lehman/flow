use crate::error::*;
use crate::variable::Variable;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub type SlotValue = Option<Box<dyn Variable>>;

#[derive(Clone)]
pub struct Slots(Arc<Vec<Mutex<SlotValue>>>);

impl Slots {
    pub fn intialize(size: usize) -> Self {
        Self(Arc::new((0..size).map(|_| Mutex::new(None)).collect()))
    }

    pub async fn acquire(&self, address: usize) -> Result<MutexGuard<'_, SlotValue>, RuntimeError> {
        Ok(self
            .0
            .get(address)
            .ok_or(RuntimeError::invalid_slot_address())?
            .lock()
            .await)
    }
}
