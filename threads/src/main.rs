use std::net::TcpListener;
use threads::ThreadPool;
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
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //thread::spawn(move || handle_stream(stream));
        pool.execute(|| handle_stream(stream));
    }
}

fn handle_stream(stream: TcpStream) {
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
