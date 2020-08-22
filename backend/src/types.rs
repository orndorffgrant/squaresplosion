use async_std::net::TcpStream;
use async_tls::server::TlsStream;
use async_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
use futures::{channel::mpsc, stream::SplitSink};
use std::error::Error;

pub type ServerResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
pub type ChannelSender<T> = mpsc::UnboundedSender<T>;
pub type ChannelReceiver<T> = mpsc::UnboundedReceiver<T>;
pub type WebSocketSender = SplitSink<WebSocketStream<TlsStream<TcpStream>>, Message>;

#[derive(Debug)]
pub enum Void {}
