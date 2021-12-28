use anyhow::Result;
use crossbeam_channel::Receiver;
use std::sync::Arc;

use super::{MapSubscribe, WsData};

pub fn subs_logic(rx: &Receiver<WsData>, map_subs: &MapSubscribe) -> Result<()> {
    // wait for the next rx and pass message along to each listener
    let msg = rx.recv()?;

    // convert struct into JSON and wrap into an Arc to pass
    let data_owner = Arc::new(serde_json::to_string(&msg)?);

    // iter through the map and send a Arc clone of the JSON string above into the on_connection loop
    let map = map_subs.lock().unwrap();
    let (_success, errors): (Vec<_>, Vec<_>) = map.iter().map(|(_, tx)| tx.send(Arc::clone(&data_owner))).partition(Result::is_ok);

    // if any errors print them and do nothing else
    errors.into_iter().for_each(|e| println!("unable to send ws msg: {}", e.unwrap_err()));
    Ok(())
}
pub async fn subs(rx: Receiver<WsData>, map_subs: MapSubscribe) {
    loop {
        if let Err(e) = subs_logic(&rx, &map_subs) {
            println!("Error sending ws data to subscribers: {}", e);
            break;
        }
    }
}
