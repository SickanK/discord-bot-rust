use base64;
use native_tls::TlsConnector;
use rand;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayEventName {
    #[serde(rename = "op")]
    op: i64,

    #[serde(rename = "d")]
    d: D,

    #[serde(rename = "s")]
    s: i64,

    #[serde(rename = "t")]
    t: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct D {
    heartbeat_interval: i64,
    _trace: [String; 1],
}

fn main() {
    let d: [u8; 16] = rand::random();
    let rstr: String = base64::encode(&d);
    let connector = TlsConnector::new().unwrap();
    let string = String::from(
        format!("GET /?v=9 HTTP/1.1\r\nHost: gateway.discord.gg\r\nAuthorization: Bot ODQ3OTQ5NTAwNTQxODI5MTMw.YLFggw.4moFWlMQgiX6nc5Q2PMRPCENv-8\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: {}\r\n\r\n", &rstr)
    );

    let stream = TcpStream::connect("gateway.discord.gg:443").unwrap();
    let mut stream = connector.connect("gateway.discord.gg", stream).unwrap();

    stream.write(string.as_bytes()).unwrap();

    thread::spawn(move || loop {
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        if remove_zero(String::from_utf8_lossy(&buf).to_string()).is_ok() {
            let g: GatewayEventName =
                serde_json::from_str(
                    remove_zero(String::from_utf8_lossy(&buf).to_string()).ok()
                );
            println!("{:?}", g);
        };
    });

    loop {}
}

fn remove_zero(bitties: String) -> Result<String, ()> {
    for n in 0..bitties.len() {
        if bitties.chars().into_iter().nth(n) == Some('\u{0}') {
            return Ok(bitties.split_at(n).0.to_string());
        }
    }

    Err(())
}
