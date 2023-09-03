use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener , TcpStream},
    time::Duration,
    thread,
    fs,
};
use web_server::ThreadPool;

fn main() {
    let requests = TcpListener::bind("127.0.0.1:3333").unwrap();
    let thread_pool = ThreadPool::new(4);
    for request in requests.incoming().take(2) {
        let request = request.unwrap();

        thread_pool.execute(|| {
            handle_request(request);
        });
    }
}

fn handle_request(mut stream : TcpStream) {
    let request = BufReader::new(&stream);
    let request_line = request.lines().next().unwrap().unwrap();
    let (status_line, html) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK" , "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK" , "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND" , "NotFound.html"),
    };
    let html_doc = fs::read_to_string(html).unwrap();
    let length = html_doc.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html_doc}");
    stream.write_all(response.as_bytes()).unwrap();

}
