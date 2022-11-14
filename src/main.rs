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
        let request_line = buf_reader.lines().next().unwrap().unwrap(); // read the first line, fail if it doesn't exist or process the result
        
        // ... process the request line
        let (status_line, filename) = if request_line == "GET / HTTP/1.1" { // check if the request is for the root
            ("HTTP/1.1 200 OK", "hello.html") // return tuple with status line and filename for deconstructed assignment
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
    
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();
        }
        println!("Shutting down.");
    }
