use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

pub mod ws_open_sockets;
pub mod ws_subscribe;

type Tx = Sender<Arc<String>>;
type MapSubscribe = Arc<Mutex<HashMap<SocketAddr, Arc<Tx>>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WsData {
    pub count: u64,
    pub message: String,
}

// each web socket subscriber will get their own connection
async fn handle_connection(map: MapSubscribe, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    println!("ws connected: {}", &addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await.expect("Error during the websocket handshake occurred");
    let (mut outgoing, _) = ws_stream.split();

    if let Err(e) = ws_open_sockets::on_connection(&mut outgoing, map, addr).await {
        println!("Error on WebSocket {}: {}", addr, e);
        // Send error message to client
        outgoing.send(Message::Text(format!("Error: {}", e))).await.expect("Failed to close");
        // Close the websocket
        outgoing.close().await.expect("Failed to close");
    };
}

pub async fn ws_init(rx: Receiver<WsData>, addr: &str) -> Result<(), IoError> {
    // Create a new state to represent each connection in a HashMap
    let state = MapSubscribe::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on
    // Panic program if this fails
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Web Socket listening on: {}", addr);

    // spawn new thread that will receive the WsData message from the main thread every second
    tokio::spawn(ws_subscribe::subs(rx, Arc::clone(&state)));

    // spawn new thread each time a client connects
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(Arc::clone(&state), stream, addr));
    }

    Ok(())
}
