use std::net::TcpListener;
use std::{
    io::{
        BufReader,
        prelude::*,
    },
    net::TcpStream,
    thread,
    time::Duration,
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

fn handle_stream2(stream: TcpStream) {
    let buff = BufReader::new(stream);
    let request = buff.lines().next().unwrap().unwrap();
    let (status, filename) = match request {
        ref line if line == "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        ref line if line == "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 Not Found", "/404.html"),
    };
}

fn handle_stream(stream: TcpStream) {
    let reader = BufReader::new(stream);
    let http_request: Vec<_> = reader.lines().map(|line| line.unwrap()).take_while(|line| !line.is_empty()).collect();
    println!("{:#?}", http_request);
}
