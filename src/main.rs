mod lib;

use std::{fs, io};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use futures::AsyncWriteExt;
use crate::lib::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(false).expect("Cannot set non-blocking");
    let pools = ThreadPool::new(5);
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //
    //     pools.execute(|| {
    //         handle_connection(stream);
    //     });
    // }
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // do something with the TcpStream
                println!("OK");
                handle_connection(s);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Decide if we should exit
                //break;
                // Decide if we should try to accept a connection again
                println!("Err");
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
