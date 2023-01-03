use std::net::{TcpStream, TcpListener};
use std::io;
use std::io::{Read, Write};

fn widen_string(src: &String) -> String {
    let mut wide: String = String::new();
    for c in src.chars() {
        wide.push(c);
        wide.push(' ');
    }
    wide
}

fn handle_connection(stream: &mut TcpStream) -> io::Result<()> {
    let mut buf = [0; 2048];
    stream.read(&mut buf[..])?;
    let req_string = std::str::from_utf8(&buf[..]).unwrap().to_string();
    let widened = widen_string(&req_string);
    let resp = widened.as_bytes();
    stream.write(resp)?;
    Ok(())
}
fn main() -> io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:3333")?;
    for connection in server.incoming() {
        match connection {
            Ok(mut stream) => {
                handle_connection(&mut stream)?;            
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }    
    Ok(())
}
