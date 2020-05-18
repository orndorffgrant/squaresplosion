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

use serde::{Deserialize, Serialize};
use serde_json::to_string;

use std::{
    collections::hash_map::{Entry, HashMap},
    error::Error,
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
        conn_id: usize,
        player_id: String,
        player_name: String,
        room_name: String,
        x: u32,
        y: u32,
        ws_sender: WebSocketSender,
        shutdown_receiver: ChannelReceiver<Void>
    },
    PlayerMove {
        from_id: usize,
        player_id: String,
        x: u32,
        y: u32,
    },
}

struct PlayerState {
    id: String,
    name: String,
    score: u64,
}
struct CellState {
    player_id: String,
}
struct SquareRoomState {
    name: String,
    player_senders: Vec<ChannelSender<Message>>,
    player_state: HashMap<String, PlayerState>,
    room_state: HashMap<u32, Box<HashMap<u32, CellState>>>,
}
impl SquareRoomState {
    fn new(name: &str) -> SquareRoomState {
        SquareRoomState{
            name: name.to_string(),
            player_senders: Vec::new(),
            player_state: HashMap::new(),
            room_state: HashMap::new(),
        }
    }
    fn update_cell(&mut self, id: &str, x: u32, y: u32) -> &SquareRoomState {
        let row = self.room_state.entry(x).or_insert_with(|| {
            Box::new(HashMap::new())
        });
        let cell = row.entry(y).or_insert_with(|| {
            CellState{
                player_id: id.to_string(),
            }
        });
        cell.player_id = id.to_string();
        self
    }
    fn add_player(&mut self, id: &str, name: &str, x: u32, y: u32, sender: ChannelSender<Message>) {
        self.player_senders.push(sender);
        self.player_state.insert(id.to_string(), PlayerState{
            id: id.to_string(),
            name: name.to_string(),
            score: 1,
        });
        self.update_cell(id, x, y);
    }
    fn update_player(&mut self, id: &str, x: u32, y: u32) {
        self.update_cell(id, x, y);
        let curr_player_state = self.player_state.get_mut(id).unwrap();
        curr_player_state.score += 1;
    }
}

async fn event_broker_task<'a>(incoming_events: ChannelReceiver<Event>) -> Result<()> {
    let (disconnect_sender, mut disconnect_receiver) = mpsc::unbounded::<usize>();
    let mut rooms: HashMap<String, SquareRoomState> = HashMap::new();
    let mut conns: HashMap<usize, String> = HashMap::new();

    let mut incoming_events = incoming_events.fuse();

    loop {
        let event = select! {
            event = incoming_events.next().fuse() => match event {
                None => break,
                Some(event) => event,
            },
            disconnect = disconnect_receiver.next().fuse() => {
                let disconnected_conn = disconnect.unwrap();
                // TODO check if room is empty
                conns.remove(&disconnected_conn);
                continue;
            },
        };

        match event {
            Event::NewConnection{
                conn_id,
                player_id,
                player_name,
                room_name,
                x,
                y,
                ws_sender,
                shutdown_receiver
            } => {
                let room = rooms.entry(room_name.to_string()).or_insert_with(|| {
                    SquareRoomState::new(room_name.as_str())
                });

                let (conn_outgoing_sender, conn_outgoing_receiver) = mpsc::unbounded();
                room.add_player(player_id.as_str(), player_name.as_str(), x, y, conn_outgoing_sender);

                conns.insert(conn_id, room_name);
                task::spawn(connection_sender_task(conn_id, ws_sender, disconnect_sender.clone(), conn_outgoing_receiver, shutdown_receiver));
            },
            Event::PlayerMove { from_id, player_id, x, y } => {
                let room_name = match conns.get(&from_id) {
                    Some(r_ref) => r_ref,
                    None => {
                        eprintln!("cant find room for conn_id {}", from_id);
                        continue;
                    }
                };
                let room = match rooms.get_mut(room_name) {
                    Some(r_ref) => r_ref,
                    None => {
                        eprintln!("cant find room for conn_id {}", from_id);
                        continue;
                    }
                };
                room.update_player(&player_id, x, y);

                // TODO create room state message and send it
                for mut sender in &room.player_senders {
                    sender.send(Message::Text("hello".to_string())).await?;
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

#[derive(Serialize, Deserialize)]
struct JoinRoomMessage {
    id: String,
    player_name: String,
    room_name: String,
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize)]
struct PlayerMoveMessage {
    x: u32,
    y: u32,
}

async fn connection_task<'a>(
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

    let room_join_msg: Message = match incoming.next().await
     {
        Some(m) => m?,
        None => {
            eprintln!("connection didn't send room join msg");
            return Ok(());
        },
    };

    let room_join: JoinRoomMessage = serde_json::from_str(room_join_msg.to_string().as_str())?;

    let player_id = room_join.id;

    match event_broker.send(Event::NewConnection{
        conn_id,
        player_id: player_id.clone(),
        player_name: room_join.player_name,
        room_name: room_join.room_name,
        x: room_join.x,
        y: room_join.y,
        ws_sender: outgoing,
        shutdown_receiver: connection_shutdown_receiver,
    }).await {
        Ok(h) => h,
        Err(e) => {
            eprintln!("event broker down {}", e);
            return Ok(())
        }
    };

    while let Some(msg) = incoming.next().await {
        let msg = msg?;
        let move_msg: PlayerMoveMessage = serde_json::from_str(msg.to_string().as_str())?;
        match event_broker.send(Event::PlayerMove{
            from_id: conn_id,
            player_id: player_id.clone(),
            x: move_msg.x,
            y: move_msg.y,
        }).await {
            Ok(h) => h,
            Err(e) => {
                eprintln!("event broker down {}", e);
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

    // TODO serialize as json
    loop {
        select! {
            msg = messages.next().fuse() => match msg {
                Some(msg) => match ws_sender.send(msg).await {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("{}", e);
                        return Ok(())
                    }
                },
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


