use std::io::{prelude::*, BufReader};
use std::net::{TcpStream};


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    let sample = "test".as_bytes();
    stream.write_all(sample).unwrap();
    println!("Hello, world!");
}
