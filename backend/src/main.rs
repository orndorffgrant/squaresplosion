mod internal_messages;
mod ws_messages;
mod state;
mod types;
mod tasks;

use async_std::{
    net::TcpListener,
    task,
};

use futures::{
    channel::mpsc,
    stream::{
        StreamExt,
    },
};

fn main() -> types::ServerResult<()> {
    task::block_on(run())
}

async fn run() -> types::ServerResult<()> {
    let server_addr = "0.0.0.0:9999";

    let mut curr_conn_id: usize = 0;

    let server = TcpListener::bind(&server_addr).await?;
    println!("Listening on {}", server_addr);

    let (event_broker_sender, event_broker_receiver) = mpsc::unbounded();

    let event_broker_task_handle = task::spawn(tasks::event_broker::run(event_broker_receiver));

    let mut incoming = server.incoming();

    while let Some(stream) = incoming.next().await {
        task::spawn(tasks::conn_reader::run(stream?, event_broker_sender.clone(), curr_conn_id));
        curr_conn_id += 1;
    }

    drop(event_broker_sender);
    event_broker_task_handle.await?;

    Ok(())
}
