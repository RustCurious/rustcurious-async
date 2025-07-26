use std::io::{self, Read, Write};
use std::net::{TcpStream, TcpListener};

fn main() -> io::Result<()> {
    let addr = "localhost:3000";
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr)?;
    loop {
        let (stream, _) = listener.accept()?;
        std::thread::spawn(move || {
            handle_client(stream);
        });
    }
}

fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    println!("Client {} connected", peer);
    if let Err(err) = echo(&mut stream) {
        println!("Client {} error: {}", peer, err);
    }
    println!("Client {} disconnected", peer);
}

fn echo(stream: &mut TcpStream) -> io::Result<()> {
    stream.write_all(b"Hello!\n")?;
    let mut buf = [0u8; 4096];
    loop {
        let len = stream.read(&mut buf)?;
        if len == 0 {
            break;
        }
        stream.write_all(&buf[..len])?;
    }
    Ok(())
}
