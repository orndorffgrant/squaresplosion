use std::io::Error;
use std::sync::Arc;

use async_std::net::{TcpListener, TcpStream};
use async_std::sync::Mutex;
use async_std::task;

use async_tungstenite::tungstenite;
use async_tungstenite::tungstenite::protocol::Message;
use async_tungstenite::WebSocketStream;

use futures::prelude::*;
use futures::stream::SplitSink;

struct Connection {
    id: u64,
    outgoing: SplitSink<WebSocketStream<TcpStream>, Message>,
}

async fn accept_connection(id: u64, conns: Arc<Mutex<Vec<Connection>>>, stream: TcpStream) -> Result<(), tungstenite::error::Error> {
    let ws_stream = async_tungstenite::accept_async(stream).await?;
    println!("New conn {}", id);

    let (outgoing, mut incoming) = ws_stream.split();

    conns.lock().await.push(Connection{id, outgoing});

    while let Some(msg) = incoming.next().await {
        let m = msg?;
        let mut i = 0;
        // Def room for race conditions and OOB indexing here
        let conns_len = conns.lock().await.len();
        while i < conns_len {
            let c = &mut conns.lock().await[i];
            if c.id != id {
                c.outgoing.send(m.clone()).await?;
            }
            i = i + 1;
        }
    }

    Ok(())
}

async fn run() -> Result<(), Error> {
    let server_addr = "0.0.0.0:9999".to_string();

    let server = TcpListener::bind(&server_addr).await.expect("failed to bind to addr");
    println!("Listening on {}", server_addr);

    let mut curr_id = 0;
    let conns = Arc::new(Mutex::new(Vec::new()));

    while let Ok((stream, _)) = server.accept().await {
        curr_id = curr_id + 1;
        task::spawn(accept_connection(curr_id, conns.clone(), stream));
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    task::block_on(run())
}
