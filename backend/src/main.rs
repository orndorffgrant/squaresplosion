use std::io::Error;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use futures::prelude::*;

async fn accept_connection(stream: TcpStream) {
    let peer_addr = stream.peer_addr().expect("peer should have address");
    println!("New connection from {}", peer_addr);

    let ws_stream = async_tungstenite::accept_async(stream).await.expect("error during ws handshake");
    println!("Successful WS handshake");

    let (write, read) = ws_stream.split();

    read.forward(write).await.expect("failed to forward message back to sender");
}

async fn run() -> Result<(), Error> {
    let server_addr = "0.0.0.0:9999".to_string();

    let server = TcpListener::bind(&server_addr).await.expect("failed to bind to addr");
    println!("Listening on {}", server_addr);

    while let Ok((stream, _)) = server.accept().await {
        task::spawn(accept_connection(stream));
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    task::block_on(run())
}
