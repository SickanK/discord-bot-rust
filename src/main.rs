pub mod util;
pub mod websockets;
use std::thread;
use std::time;

use websockets::{frame::WSFrame, WebSocket};

//use self::websockets::get_length;

fn main() {
    let mut ws: WebSocket = WebSocket::new("gateway.discord.gg");
    let (tx, rx) = ws.initialize();
    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_millis(4000));
        tx.send(WSFrame::new(
            true,
            true,
            websockets::opcode::RFC6455Opcode::Binary,
            14,
            String::from("{\"op\":1,\"d\":2}"),
        ))
        .unwrap();
    });

    loop {
        println!("{:?}", rx.recv().unwrap());
    }
}
