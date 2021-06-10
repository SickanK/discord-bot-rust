pub mod discord;
pub mod util;
pub mod websockets;
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicU32, Arc, Mutex};

use discord::{
    handle_response,
    opcode::DiscordOpcode,
    payload::{GatewayIdentify, GatewayIdentifyData, GatewayIdentifyDataProperties},
};
use websockets::{frame::WSFrame, WebSocket};

extern crate serde_derive;

//use self::websockets::get_length;

#[derive(Serialize, Deserialize)]
struct Settings {
    bot_token: String,
}

fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let settings = settings.try_into::<Settings>().unwrap();

    let mut ws: WebSocket = WebSocket::new("gateway.discord.gg");
    let (tx, rx) = ws.initialize();
    let sequence: Arc<Mutex<AtomicU32>> = Arc::new(Mutex::new(AtomicU32::new(0)));

    let identify_payload_struct = GatewayIdentify {
        op: DiscordOpcode::Identify.get_u8(),
        d: GatewayIdentifyData {
            token: settings.bot_token,
            intents: 513,
            properties: GatewayIdentifyDataProperties {
                os: String::from("linux"),
                browser: String::from("my_library"),
                device: String::from("my_library"),
            },
        },
    };

    let identify_payload = serde_json::to_string(&identify_payload_struct).unwrap();

    let identify_frame = WSFrame {
        fin: true,
        mask: true,
        opcode: websockets::opcode::RFC6455Opcode::Text,
        payload_length: identify_payload.len(),
        payload: identify_payload,
    };

    tx.send(identify_frame).unwrap();

    loop {
        let frame = rx.recv().unwrap();
        handle_response(frame, sequence.clone(), tx.clone())
    }
}
