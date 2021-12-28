use anyhow::Result;
use crossbeam_channel::bounded;
use tokio::time;
use web_socket::WsData;

mod web_socket;

const WEB_SOCKET_ADDR: &'static str = "127.0.0.1:34534";

#[tokio::main]
async fn main() -> Result<()> {
    // open a channel between the main thread and the web socket
    // tx is the transmitter that will be used in the below loop
    // rx is the receiver that will be listening in the web_socket thread
    let (tx, rx) = bounded(1);

    // create a new thread for the web_socket logic and pass in the rx side of the channel
    tokio::spawn(async move {
        web_socket::ws_init(rx, &WEB_SOCKET_ADDR).await.expect("Unable to initialize WebSocket listener");
    });

    // Loop every 1 second and send simple JSON message to any listeners
    let mut count = 1;
    loop {
        let ws_data = WsData {
            count,
            message: format!("message number: {}", count),
        };

        // send WsData across to web_socket thread
        tx.send(ws_data)?;

        // sleep for 1 second
        time::sleep(time::Duration::from_millis(1000)).await;

        // increment the count by one
        count += 1;
    }
}
