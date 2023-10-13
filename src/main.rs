use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listner = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

/*
HTTP Request format:
    Method Request-URI HTTP-Version CRLF
    HTTP Request format:
    headers CRLF
    message-body

HTTP Response format:
    HTTP-Version Status-Code Reason-Phrase CRLF         // status line
    headers CRLF                                        //
    message-body                                        //

e.g.

*/

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (status_line, filename) = if http_request[0] == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "pages/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "pages/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap()
}
