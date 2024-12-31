use std::collections::HashMap;
use crate::store_value::StoreValue;

pub type Result<T> = std::result::Result<T, &'static str>;

pub struct Store {
  store: HashMap<String, StoreValue>,
}

impl Store {
  fn new() -> Self {
    Self {
      store: HashMap::new(),
    }
  }

  fn get(&self, key: String) -> Result<String> {
    let fetch = self.store.get(&key);
    match fetch {
      Some(store_value) => Ok(store_value.value.clone()),
      None => Err("Key not found")
    } 
  }

  fn set(&mut self, key: String, value: String, ttl: i64) -> Result<String> {
    let store_value = StoreValue {
      value: value,
      ttl: ttl,
    };
    let insertion = self.store.insert(key, store_value);
    match insertion {
      Some(inserted_value) => Ok(inserted_value.value.clone()),
      None => Err("Instertion failed")
    }
  }

  fn delete(&mut self, key: String) -> Result<()> {
    let deletion = self.store.remove(&key);
    match deletion {
      Some(_deleted_value) => Ok(()),
      None => Err("Deletion failed")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_and_set() {
    let store = Store::new();
    store.set("key".to_string(), "value".to_string(), 1000).unwrap();
    let store_value = store.get("key".to_string()).unwrap();
    assert_eq!(store_value, "value");
  }

  #[test]
  fn test_get_failure() {}

  #[test]
  fn test_set_failure() {}

  #[test]
  fn test_delete() {
    let store = Store::new();
    store.set("key".to_string(), "value".to_string(), 1000).unwrap();
    store.delete("key".to_string()).unwrap();
    let store_value = store.get("key".to_string());
    assert_eq!(store_value.is_err(), true);
  }
}