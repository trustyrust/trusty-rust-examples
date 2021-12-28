use anyhow::Result;
use crossbeam_channel::bounded;
use futures::stream::SplitSink;
use futures_util::SinkExt;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use super::{MapSubscribe};

pub async fn on_connection(outgoing: &mut SplitSink<WebSocketStream<TcpStream>, Message>, map: MapSubscribe, addr: SocketAddr) -> Result<()> {
    // created the crossbeam channel to communicate with subs_logic
    let (tx, rx) = bounded(1);
    map.lock().unwrap().insert(addr, Arc::new(tx));

    // blocking for all future subscriptions
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

    // hangup the channel
    map.lock().unwrap().remove(&addr);
    println!("ws disconnected: {} -> open connections: {}", &addr, Arc::strong_count(&map) - 3);
    Ok(())
}
