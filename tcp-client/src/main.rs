use std::net::TcpStream;
use std::io;
use std::io::{Read, Write};
fn main() -> io::Result<()> {
    loop {
        println!("Say something (enter quit to exit application): ");
        let mut src: String = String::new();
        io::stdin().read_line(&mut src)?;
        src = src.trim().to_string();
        if src == "quit" {
            break;
        }
        let mut stream = TcpStream::connect("127.0.0.1:3333")?;
        stream.write(src.as_bytes())?;
        let mut buf = [0; 2048];
        stream.read(&mut buf[..])?;
        println!("{}", std::str::from_utf8(&buf[..])
                 .unwrap()
                 .to_string()
                 .trim());
    }
    Ok(())
}
