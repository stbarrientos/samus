mod store;
mod store_value;
mod server;
use server::Server;
use store::Store;

fn main() {
  let test_key = "test_key".to_string();
  let test_value = "test_value".to_string();
  let test_ttl = 0;
  let mut store = Store::new();
  store.set(&test_key, &test_value, &test_ttl).unwrap();
  let mut server = Server::new(6666, &mut store);
  server.start().unwrap();
}


