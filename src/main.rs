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
    
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; MAX_REQUEST_LENGTH];
    stream.read(&mut buf);
    if DEBUG {
        println!("Got request: {}", std::str::from_utf8(&buf).unwrap());
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
