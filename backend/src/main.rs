use async_std::{
    net::{TcpListener, TcpStream},
    task,
};

use async_tungstenite::{
    tungstenite::protocol::Message,
    WebSocketStream,
    accept_async,
};

use futures::{
    channel::mpsc,
    select,
    sink::SinkExt,
    stream::{
        StreamExt,
        SplitSink,
    },
    FutureExt,
};

use std::{
    collections::hash_map::{Entry, HashMap},
    error::Error,
    future::Future,
};


type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
type ChannelSender<T> = mpsc::UnboundedSender<T>;
type ChannelReceiver<T> = mpsc::UnboundedReceiver<T>;
type WebSocketSender = SplitSink<WebSocketStream<TcpStream>, Message>;

#[derive(Debug)]
enum Void {}

fn main() -> Result<()> {
    task::block_on(run_task())
}

async fn run_task() -> Result<()> {
    let server_addr = "0.0.0.0:9999";

    let mut curr_conn_id: usize = 0;

    let server = TcpListener::bind(&server_addr).await?;
    println!("Listening on {}", server_addr);

    let (event_broker_sender, event_broker_receiver) = mpsc::unbounded();

    let event_broker_task_handle = task::spawn(event_broker_task(event_broker_receiver));

    let mut incoming = server.incoming();

    while let Some(stream) = incoming.next().await {
        task::spawn(connection_task(stream?, event_broker_sender.clone(), curr_conn_id));
        curr_conn_id += 1;
    }

    drop(event_broker_sender);
    event_broker_task_handle.await?;

    Ok(())
}


enum Event {
    NewConnection {
        id: usize,
        ws_sender: WebSocketSender,
        shutdown_receiver: ChannelReceiver<Void>
    },
    Message {
        from_id: usize,
        msg: Message,
    },
}

async fn event_broker_task(incoming_events: ChannelReceiver<Event>) -> Result<()> {
    let (disconnect_sender, mut disconnect_receiver) = mpsc::unbounded::<usize>();
    let mut conns: HashMap<usize, ChannelSender<Message>> = HashMap::new();
    let mut incoming_events = incoming_events.fuse();

    loop {
        let event = select! {
            event = incoming_events.next().fuse() => match event {
                None => break,
                Some(event) => event,
            },
            disconnect = disconnect_receiver.next().fuse() => {
                let disconnected_conn = disconnect.unwrap();
                conns.remove(&disconnected_conn);
                continue;
            },
        };
        match event {
            Event::NewConnection { id, ws_sender, shutdown_receiver } => {
                let existing_entry = conns.entry(id);
                match existing_entry {
                    Entry::Occupied(_) => (),
                    Entry::Vacant(entry) => {
                        let (conn_outgoing_sender, conn_outgoing_receiver) = mpsc::unbounded();
                        entry.insert(conn_outgoing_sender);
                        spawn_and_log_error(connection_sender_task(id, ws_sender, disconnect_sender.clone(), conn_outgoing_receiver, shutdown_receiver));
                    },
                }
            },
            Event::Message { from_id, msg } => {
                for other_key in conns.keys() {
                    if *other_key != from_id {
                        if let Some(mut other_ws_sender) = conns.get(other_key) {
                            other_ws_sender.send(msg.clone()).await?;
                        }
                    }
                }
            },
        }
    }

    drop(conns);
    drop(disconnect_sender);
    while let Some(_id) = disconnect_receiver.next().await {
    }

    Ok(())
}

async fn connection_task(
    stream: TcpStream,
    mut event_broker: ChannelSender<Event>,
    conn_id: usize,
) -> Result<()> {
    let peer_addr = match stream.peer_addr() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(())
        }
    };
    let ws_stream = match accept_async(stream).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(())
        }
    };
    println!("New conn ({}) from {}", conn_id, peer_addr);

    let (outgoing, mut incoming) = ws_stream.split();

    let (_connection_shutdown_sender, connection_shutdown_receiver) = mpsc::unbounded::<Void>();

    match event_broker.send(Event::NewConnection{
        id: conn_id,
        ws_sender: outgoing,
        shutdown_receiver: connection_shutdown_receiver,
    }).await {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(())
        }
    };

    while let Some(msg) = incoming.next().await {
        match event_broker.send(Event::Message{
            from_id: conn_id,
            msg: msg?,
        }).await {
            Ok(h) => h,
            Err(e) => {
                eprintln!("{}", e);
                return Ok(())
            }
        }
    }

    Ok(())
}

async fn connection_sender_task(
    id:  usize,
    mut ws_sender: WebSocketSender,
    mut disconnect_sender: ChannelSender<usize>,
    message_receiver: ChannelReceiver<Message>,
    shutdown_receiver: ChannelReceiver<Void>,
) -> Result<()> {
    let mut messages = message_receiver.fuse();
    let mut shutdown = shutdown_receiver.fuse();

    loop {
        select! {
            msg = messages.next().fuse() => match msg {
                Some(msg) => ws_sender.send(msg).await?,
                None => break,
            },
            void = shutdown.next().fuse() => match void {
                Some(void) => match void {},
                None => break,
            },
        }
    }

    disconnect_sender.send(id).await.unwrap();
    Ok(())
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()> where F: Future<Output = Result<()>> + Send + 'static {
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}
