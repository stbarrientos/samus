#[derive(Debug)]
pub struct StoreValue {
  pub value: String,
  pub ttl: i64,
}

impl Clone for StoreValue {
  fn clone(&self) -> StoreValue {
    StoreValue {
      value: self.value.clone(),
      ttl: self.ttl,
    }
  }
}
