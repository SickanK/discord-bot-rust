pub mod discord;
pub mod http;
pub mod util;
pub mod websockets;
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicU32, Arc, Mutex};

use discord::{handle_response, identify_frame};
use websockets::WebSocket;

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate num_derive;

#[derive(Serialize, Deserialize)]
struct Settings {
    bot_token: String,
}

fn main() {
    let mut ws: WebSocket = WebSocket::new("gateway.discord.gg");
    let tx = ws.initialize();
    let sequence: Arc<Mutex<AtomicU32>> = Arc::new(Mutex::new(AtomicU32::new(0)));

    tx.send(identify_frame()).unwrap();

    loop {
        let frame = ws.recv_receiver.recv().unwrap();
        handle_response(frame, sequence.clone(), tx.clone())
    }
}
