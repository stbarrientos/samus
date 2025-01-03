use std::collections::HashMap;
use crate::store_value::StoreValue;

pub type Result<T> = std::result::Result<T, &'static str>;

pub struct Store {
  store: HashMap<String, StoreValue>,
}

impl Store {
  pub fn new() -> Store {
    Store {
      store: HashMap::new(),
    }
  }

  pub fn get(&self, key: &String) -> Result<String> {
    let fetch = self.store.get(key);
    match fetch {
      Some(store_value) => Ok(store_value.value.clone()),
      None => Err("Key not found")
    } 
  }

  pub fn set(&mut self, key: &String, value: &String, ttl: &i64) -> Result<String> {
    let store_value = StoreValue {
      value: value.clone(),
      ttl: ttl.clone(),
    };
    let insertion = self.store.insert(key.clone(), store_value.clone());
    match insertion {
      Some(_) => Ok(value.clone()),
      None => Ok(value.clone())
    }
  }

  pub fn delete(&mut self, key: &String) -> Result<String> {
    let deletion = self.store.remove(key);
    match deletion {
      Some(deleted_value) => Ok(deleted_value.value),
      None => Ok("".to_string())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_and_set() {
    let mut store = Store::new();
    let test_key = "key".to_string();
    let test_value = "value".to_string();
    let test_ttl = 1000;
    store.set(&test_key, &test_value, &test_ttl).unwrap();
    let store_value = store.get(&test_key).unwrap();
    assert_eq!(store_value, "value");
  }

  #[test]
  fn test_delete() {
    let mut store = Store::new();
    let test_key = "key".to_string();
    let test_value = "value".to_string();
    let test_ttl = 1000;
    store.set(&test_key, &test_value, &test_ttl).unwrap();
    store.delete(&test_key).unwrap();
    let store_value = store.get(&test_key);
    assert_eq!(store_value.is_err(), true);
  }
}