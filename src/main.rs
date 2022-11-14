use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // this is a blocking call

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream); // construct a buffered reader
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap()) // this yields a Result<String, io::Error>
            .take_while(|line| !line.is_empty()) // this weeds out the empty lines
            .collect(); // this makes a Vec<String>
            // println!("Request: {:#?}", http_request);
        
        let status_line = "HTTP/1.1 200 OK"; // this is the status line 200 OK
        let contents = fs::read_to_string("hello.html").unwrap(); // this is the file we are reading
        let length = contents.len(); // this is the length of the file

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap(); // write the response to the stream
    }

    println!("Shutting down.");
}
