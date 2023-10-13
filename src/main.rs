use std::{
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

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    println!("{:?}", response);
    stream.write_all(response.as_bytes()).unwrap()

    // println!("Request : {:#?}", http_request);
    // println!("==============================================================================")
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
