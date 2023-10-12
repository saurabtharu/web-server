use std::net::TcpListener;

fn main() {
    let listner = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        println!("The connection established: {:?}", stream);
    }
}
