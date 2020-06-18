#![recursion_limit="256"]

mod internal_messages;
mod ws_messages;
mod state;
mod types;
mod tasks;

use async_std::{
    net::TcpListener,
    task,
    io,
};
use async_tls::TlsAcceptor;
use futures::{
    channel::mpsc,
    stream::{
        StreamExt,
    },
};
use log::{
    trace,
    info,
};
use pretty_env_logger;
use std::{
    fs::File,
    io::BufReader,
    net::ToSocketAddrs,
    path::PathBuf,
    sync::Arc,
};
use structopt::StructOpt;
use rustls::{
    NoClientAuth,
    ServerConfig
};

#[derive(StructOpt)]
struct Options {
    addr: String,

    #[structopt(short = "c", long = "cert", parse(from_os_str))]
    cert: PathBuf,

    #[structopt(short = "k", long = "key", parse(from_os_str))]
    key: PathBuf,
}

fn main() -> types::ServerResult<()> {
    pretty_env_logger::init();
    trace!("starting up");
    task::block_on(run())
}

async fn run() -> types::ServerResult<()> {
    let options = Options::from_args();

    let server_addr = options
        .addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;

    let tls_config = load_config(&options)?;

    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    let mut curr_conn_id: usize = 0;

    let server = TcpListener::bind(&server_addr).await?;
    info!("Listening on {}", server_addr);

    let (event_broker_sender, event_broker_receiver) = mpsc::unbounded();

    let event_broker_task_handle = task::spawn(tasks::event_broker::run(event_broker_receiver));

    let mut incoming = server.incoming();

    while let Some(stream) = incoming.next().await {
        task::spawn(tasks::conn_reader::run(stream?, acceptor.clone(), event_broker_sender.clone(), curr_conn_id));
        curr_conn_id += 1;
    }

    drop(event_broker_sender);
    event_broker_task_handle.await?;

    Ok(())
}

fn load_config(options: &Options) -> io::Result<ServerConfig> {
    let cert_vec = rustls::internal::pemfile::certs(&mut BufReader::new(File::open(&options.cert)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))?;

    let rsa_keys = {
        let keyfile = File::open(&options.key)?;
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::rsa_private_keys(&mut reader)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid rsa key"))?
    };

    let pkcs8_keys = {
        let keyfile = File::open(&options.key)?;
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::pkcs8_private_keys(&mut reader)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid pkcs8 key"))?
    };

    // prefer to load pkcs8 keys
    let key = {
        if !pkcs8_keys.is_empty() {
            pkcs8_keys[0].clone()
        } else {
            assert!(!rsa_keys.is_empty());
            rsa_keys[0].clone()
        }
    };

    let mut config = ServerConfig::new(NoClientAuth::new());
    config
        .set_single_cert(cert_vec, key)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    Ok(config)
}