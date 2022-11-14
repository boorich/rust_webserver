use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // create a thread pool with 4 threads

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream); // this closure is executed in a thread
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream); // construct a buffered reader
        let request_line = buf_reader.lines().next().unwrap().unwrap(); // read the first line, fail if it doesn't exist or process the result
        
        // ... process the request line
        let (status_line, filename) = match &request_line[..] { // match on the request line
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"), // if it's a GET request, return the status line and the filename
            "GET /sleep HTTP/1.1" => { // if it's a GET request to /sleep
                thread::sleep(Duration::from_secs(5)); // sleep for 5 seconds
                ("HTTP/1.1 200 OK", "hello.html") // return the status line and the filename
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"), // if it's anything else, return the status line and the filename
        };
        
        // ... process the rest of the request
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
    
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
        // ... send the response
        stream.write_all(response.as_bytes()).unwrap();
        }
        println!("Shutting down.");
    }
