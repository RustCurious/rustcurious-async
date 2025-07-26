use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "localhost:3000";
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            handle_client(stream).await;
        });
    }
}

async fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    println!("Client {} connected", peer);
    if let Err(err) = echo(&mut stream).await {
        println!("Client {} error: {}", peer, err);
    }
    println!("Client {} disconnected", peer);
}

async fn echo(stream: &mut TcpStream) -> io::Result<()> {
    stream.write_all(b"Hello!\n").await?;
    let mut buf = [0u8; 4096];
    loop {
        let len = stream.read(&mut buf).await?;
        if len == 0 {
            break;
        }
        stream.write_all(&buf[..len]).await?;
    }
    Ok(())
}
