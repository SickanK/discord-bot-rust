pub mod dispatch;
pub mod opcode;
pub mod payload;

use serde_json;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

use self::{opcode::DiscordOpcode, payload::GatewayHeartbeat};
use crate::discord::dispatch::DiscordDispatch;
use crate::discord::opcode::handle_heartbeat;
use crate::{
    discord::opcode::Opcode,
    websockets::{frame::WSFrame, opcode::RFC6455Opcode},
};

pub fn heartbeat_frame(seq: &AtomicU32) -> WSFrame {
    let gateway_heartbeat = GatewayHeartbeat {
        op: DiscordOpcode::Heartbeat.get_u8(),
        d: seq.load(Ordering::Relaxed),
    };
    let payload = serde_json::to_string(&gateway_heartbeat).unwrap();

    seq.store(1, Ordering::SeqCst);

    WSFrame::new(true, true, RFC6455Opcode::Binary, payload.len(), payload)
}
//

pub fn handle_response(frame: WSFrame, seq: Arc<Mutex<AtomicU32>>, tx: Sender<WSFrame>) {
    let opcode: Opcode = serde_json::from_str(&frame.payload).unwrap();

    match DiscordOpcode::from_u8(opcode.op) {
        DiscordOpcode::Dispatch => DiscordDispatch::from_frame(frame).unwrap().run(),
        DiscordOpcode::Heartbeat => todo!(),
        DiscordOpcode::Identify => todo!(),
        DiscordOpcode::PresenceUpdate => todo!(),
        DiscordOpcode::VoiceStateUpdate => todo!(),
        DiscordOpcode::Resume => todo!(),
        DiscordOpcode::Reconnect => todo!(),
        DiscordOpcode::RequestGuildMembers => todo!(),
        DiscordOpcode::InvalidSession => todo!(),
        DiscordOpcode::Hello => handle_heartbeat(frame, seq.clone(), tx.clone()),
        DiscordOpcode::HeartbeatACK => println!("Pog heartbeat"),
    }
}
