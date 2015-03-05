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
    let mut buf = [0];
    //let request: String = stream.read_to_string().unwrap();
    stream.read(&mut buf);
    match &buf[0..7] {
                     b"GET    " => {
                        stream.write_str("HTTP/1.0 200 OK\nweb_server\nContent-type: text/plain\nContent-Length: 2\nOK");
    
                 } 
                     _ => {stream.write_str("HTTP/1.0 400 Bad Request");},
   /* let mut request_split = request.as_slice().split(' ');
    //let method = request_split.next().unwrap();
    if (method.as_slice() == "GET") {
        let response: String = format!("HTTP/1.0 200 OK\nweb_server\nContent-type: text/plain\nContent-Length: 2\nOK");
        stream.write_str(response.as_slice());
    }
    else {
        stream.write_str("HTTP/1.0 400 Bad Request");
    }*/
    }
}






