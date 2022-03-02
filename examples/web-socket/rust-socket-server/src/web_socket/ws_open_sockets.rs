use super::MapSubscribe;

use anyhow::Result;
use crossbeam_channel::{bounded, Receiver};
use futures::{
    future, pin_mut,
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use futures_util::SinkExt;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

pub async fn on_connection(outgoing: &mut SplitSink<WebSocketStream<TcpStream>, Message>, incoming: &mut SplitStream<WebSocketStream<TcpStream>>, map: MapSubscribe, addr: SocketAddr) -> Result<()> {
    // created the crossbeam channel to communicate with subs_logic
    let (tx, rx) = bounded(1);
    map.lock().unwrap().insert(addr, Arc::new(tx));

    // blocking for all future subscriptions
    let r = read_client_connected(incoming);
    let s = send_msg(rx, outgoing);

    // This pins r and s to the stack to do the future::select
    pin_mut!(r, s);
    // whichever future returns first, (r) client disconnects or (s) sending fails because client is unavailable
    // should most of the time be (r) first but it happens that a client becomes unavailable in middle of transmission
    future::select(r, s).await;

    // hangup the channel
    map.lock().unwrap().remove(&addr);
    println!("ws disconnected: {} -> open connections: {}", &addr, Arc::strong_count(&map) - 3);
    Ok(())
}

async fn send_msg(rx: Receiver<Arc<String>>, outgoing: &mut SplitSink<WebSocketStream<TcpStream>, Message>) {
    loop {
        match rx.recv() {
            Ok(msg_json) => {
                // send the message to the client
                match &outgoing.send(Message::Text(msg_json.to_string())).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("err from rx.recv will now close connection: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}

async fn read_client_connected(incoming: &mut SplitStream<WebSocketStream<TcpStream>>) {
    // As long as the client is connect incoming will be able to await on next.
    loop {
        let msg = incoming.next().await;
        match msg {
            Some(msg) => match msg {
                Ok(msg) => {
                    // Only care about close messages. All other messages will be ignored
                    if msg.is_close() {
                        println!("{}", "client disconnected, close socket gracefully");
                        break;
                    }
                }
                Err(e) => {
                    println!("error on read_client_connected: {}", e);
                    break;
                }
            },
            None => {
                println!("{}", "read_client_connected received None");
                break;
            }
        }
    }
}
