use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::read_to_string;
use std::{process, thread};
use std::time::Duration;

use hello::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4).unwrap_or_else(|error| {
        println!("{}", error.info);
        process::exit(1);
    });

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_stream(stream);
        });
    }

    println!("Shutting down..");

}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer[..]).unwrap();

    // println!("Request : {}",String::from_utf8_lossy(&buffer[..]));

    // let response = String::from("Http/1.1 200 Ok /r/n/r/n");

    let get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";

    // if else 代码重构
    let (statue_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        statue_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}
