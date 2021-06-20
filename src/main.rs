pub mod discord;
pub mod http_handler;
pub mod util;
pub mod websockets;
use http::HeaderMap;
use http_handler::DiscordHttp;
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

use tokio;

#[derive(Serialize, Deserialize)]
struct Settings {
    bot_token: String,
}

#[derive(Serialize, Deserialize)]
struct Dis {
    content: String,
}

#[tokio::main]
async fn main() {
    let mut ws: WebSocket = WebSocket::new("gateway.discord.gg");
    let tx = ws.initialize();
    let sequence: Arc<Mutex<AtomicU32>> = Arc::new(Mutex::new(AtomicU32::new(0)));

    tx.send(identify_frame()).unwrap();

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let settings = settings.try_into::<Settings>().unwrap();
    let d = DiscordHttp::new(settings.bot_token);
    let string = serde_json::to_string(&Dis {
        content: String::from("hello"),
    })
    .unwrap();

    let mut hm = HeaderMap::new();
    hm.insert("Content-Type", "application/json".parse().unwrap());
    println!(
        "{:?}",
        d.request(
            "https://discord.com/api/channels/847949836849250378/messages".to_string(),
            Some(hm),
            None,
            Some(string),
        )
        .unwrap(),
    );

    loop {
        let frame = ws.recv_receiver.recv().unwrap();
        handle_response(frame, sequence.clone(), tx.clone())
    }
}
