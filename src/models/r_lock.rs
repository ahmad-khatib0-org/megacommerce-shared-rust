use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};

#[derive(Debug, Clone)]
pub struct RLock<T>(pub Arc<RwLock<T>>);

impl<T> RLock<T> {
  pub async fn get(&self) -> RwLockReadGuard<'_, T> {
    self.0.read().await
  }
}
