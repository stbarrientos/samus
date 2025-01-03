use std::{
  io::{prelude::*, Write, BufReader, Error},
  net::{TcpListener, TcpStream},
};

use crate::store::Store;

const CLOSE_CONNECTION_MESSAGE: &str = "__TERM__";

pub struct Server<'a> {
  store: &'a mut Store,
  port: i32
}

impl<'a> Server<'a> {
  pub fn new(port: i32, store: &'a mut Store) -> Server<'a> {
    Server {
      store: store,
      port: port
    }
  }

  pub fn start(&mut self) -> Result<(), Error> {
    println!("Server starting on port {}", self.port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
    println!("Server started on port {}", self.port);
    for stream in listener.incoming() {
      let mut stream = stream.unwrap();
      let result = self.handle_connection(&stream);
      match result {
        Ok(_) => {}
        Err(e) => {
          let error_message = format!("Error: {}", e.to_string());
          println!("{}", error_message);
          stream.write_all(error_message.as_bytes())?;
        }
      }
      stream.flush()?;
      stream.write_all(CLOSE_CONNECTION_MESSAGE.as_bytes())?;
    }
    Ok(())
  }

  fn handle_connection(&mut self, stream: &TcpStream) -> Result<(), Error> {
    let mut buf_reader = BufReader::new(stream); // TODO: Making this immutabe breaks the tests
    let request: Vec<_> = buf_reader.lines().map(|line| line.unwrap()).collect();
    for line in request {
      self.parse_request_line(&line, &stream)?;
    }
    Ok(())
  }

  fn parse_request_line(&mut self, line: &String, stream: &TcpStream) -> Result<(), Error> {
    let keywords: Vec<_> = line.as_str().split_whitespace().collect();
    match keywords[0] {
      "GET" => {
        self.handle_get_request(&keywords[1].to_string(), stream)?;
      }
      "SET" => {
        let ttl = keywords[3].parse::<i64>();
        match ttl {
          Ok(ttl) => {
            self.handle_set_request(
              &keywords[1].to_string(),
              &keywords[2].to_string(),
              &ttl,
              stream
            )?;
          }
          Err(_) => {
            return Err(Error::other("Unparsable TTL"));
          }
        }
      }
      "DELETE" => {
        self.handle_delete_request(&keywords[1].to_string(), stream)?;
      }
      _ => {
        return Err(Error::other("Invalid request action"));
      }
    }
    Ok(())
  }

  fn handle_get_request(&self, key: &String, mut stream: &TcpStream) -> Result<(), Error> {
    let get_result = self.store.get(&key);
    match get_result {
      Ok(get_result) => {
        stream.write_all(get_result.as_bytes())?;
        stream.write_all("\n".as_bytes())?;
        stream.flush()?;
        return Ok(());
      }
      Err(e) => {
        return Err(Error::other(e.to_string()));
      }
    }
  }

  fn handle_set_request(&mut self, key: &String, value: &String, ttl: &i64, mut stream: &TcpStream) -> Result<(), Error> {
    let set_result = self.store.set(&key, &value, &ttl);
    match set_result {
      Ok(set_result) => {
        stream.write_all(set_result.as_bytes())?;
        stream.write_all("\n".as_bytes())?;
        stream.flush()?;
        Ok(())
      }
      Err(e) => {
        Err(Error::other(e.to_string()))
      }
    }
  }

  fn handle_delete_request(&mut self, key: &String, mut stream: &TcpStream) -> Result<(), Error> {
    let delete_result = self.store.delete(&key);
    match delete_result {
      Ok(delete_result) => {
        stream.write_all(delete_result.as_bytes())?;
        stream.write_all("\n".as_bytes())?;
        stream.flush()?;
        Ok(())
      }
      Err(e) => {
        return Err(Error::other(e.to_string()));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{
    time::Duration,
    thread::sleep,
    thread,
  };

  #[test]
  fn test_get_request() {
    // Spawn our server in a seperate thread
    thread::spawn(|| {
      let test_key = "test_key".to_string();
      let test_value = "test_value".to_string();
      let test_ttl = 0;
      let mut store = Store::new();
      store.set(&test_key, &test_value, &test_ttl).unwrap();
      let mut server = Server::new(6666, &mut store);
      server.start().unwrap();
    });

    // Give our server time to wake up. This isn't ideal, but it works for now and gives me
    // confidence in the server.
    sleep(Duration::new(1, 0));

    // Start a simple client
    let mut test_stream = TcpStream::connect("127.0.0.1:6666").unwrap();
    let mut test_buffer = [0; 19];

    // Send a GET request to our server
    test_stream.write_all(b"GET test_key\n").unwrap();
    test_stream.shutdown(std::net::Shutdown::Write).unwrap();
    let mut taker = test_stream.take(19);
    taker.read(&mut test_buffer).unwrap();

    let buffer_string = std::str::from_utf8(&test_buffer).unwrap();
    assert_eq!(buffer_string, "test_value\n__TERM__");
  }
}