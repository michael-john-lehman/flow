use crate::error::RuntimeError;
use std::any::Any;
use tokio::sync::{Mutex, MutexGuard};

pub struct Slots(Vec<Mutex<Option<Box<dyn Any + Send + Sync + 'static>>>>);

impl Slots {
    pub fn initialize(size: usize) -> Self {
        Self((0..size).map(|_| Mutex::new(None)).collect())
    }
    
    pub async fn acquire(&self, address: usize) -> Result<MutexGuard<'_, Option<Box<dyn Any + Send + Sync + 'static>>>, RuntimeError> {
        let lock = self.0.get(address).ok_or(RuntimeError::invalid_address())?
            .lock()
            .await;
        Ok(lock)
    }
}
