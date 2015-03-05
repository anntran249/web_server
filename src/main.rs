use std::io;
use std::io::{TcpListener,TcpStream};
use std::io::{Listener,Acceptor};
use std::thread::Thread;
use std::io::net::tcp;

const SERVER_NAME: &'static str = "IBATs_web_server";
// max limit tends to be 8KB (Firefox), 4KB (Opera), or 2KB (IE, Safari)
const MAX_REQUEST_LENGTH: usize = 8192;

const DEBUG: bool = true;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut acceptor = listener.listen().unwrap();
    
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => {println!("error: {}", e) }
            Ok(stream) => { 
                    Thread::spawn(move|| {
                        handle_client(stream)
                    });
            }
        }
    }
    drop(acceptor);
}

// structure of request (only handling GETs)
// GET <path to file> HTTP
// <path to file> is of unknown length
//  - max length of 8192 - 9 = 8183
//  - 9 from 4 bytes from "GET " + 5 bytes from " HTTP"
    
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; MAX_REQUEST_LENGTH];
    stream.read(&mut buf);
    let get_request = |&:| { buf.starts_with("GET ".as_bytes()) };

    if DEBUG {
        println!("Got request: {}", std::str::from_utf8(&buf).unwrap());

        println!("starts_with 'GET ': {}", buf.starts_with("GET ".as_bytes()));

        println!("get_request : {}", get_request());

        let request: &[u8]  = buf.splitn(1, |&: x| *x == 0).next().unwrap();
        let request_str: &str = std::str::from_utf8(request).unwrap();
        let new_lines: &[_] = &['\n', '\r'];
        let trimmed: &str = request_str.trim_right_matches(new_lines);
        let mut splits = trimmed.split(' ');
        if splits.clone().count() == 3 {
            let method: &str = splits.next().unwrap();
            let file: &str = splits.next().unwrap();
            let ending: &str = splits.next().unwrap();
            println!("method: {}", method);
            println!("file: {}", file);
            println!("ending: {}", ending);
            println!("{}", ending.trim_matches(|&: c: char| c == '/' || c == '.' || c.is_numeric()));
        } else {
            let method: &str = "";
            let file: &str = "";
            let ending: &str = "";
        }

        println!("request: {:?}", request_str);
        println!("request2: {:?}", trimmed);
        println!("ends_with ' HTTP': {}", trimmed.ends_with(" HTTP"));
        println!("\n")
    }

    match &buf[0..4] {
        b"GET " => {
            let resp: &str = "HTTP/1.0 200 OK\nweb_server\nContent-type: text/plain\nContent-Length: 2\n\nOK\n";
            stream.write_str(resp);
        },
        _ => {
            stream.write_str("HTTP/1.0 400 Bad Request\n");
        },
    }
}
