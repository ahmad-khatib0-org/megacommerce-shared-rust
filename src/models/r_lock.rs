use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};

#[derive(Debug, Clone)]
pub struct RLock<T: ?Sized>(pub Arc<RwLock<T>>);

impl<T: ?Sized> RLock<T> {
  pub async fn get(&self) -> RwLockReadGuard<'_, T> {
    self.0.read().await
  }
}
