use super::{heartbeat_frame, payload::GatewayHello};
use crate::websockets::frame::WSFrame;
use core::time;
use std::{
    sync::{atomic::AtomicU32, mpsc::Sender, Arc, Mutex},
    thread,
};

#[derive(Serialize, Deserialize)]
pub struct Opcode {
    pub op: u8,
}

pub enum DiscordOpcode {
    Dispatch,
    Heartbeat,
    Identify,
    PresenceUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatACK,
}

impl DiscordOpcode {
    pub fn from_u8(code: u8) -> Self {
        match code {
            0 => DiscordOpcode::Dispatch,
            1 => DiscordOpcode::Heartbeat,
            2 => DiscordOpcode::Identify,
            3 => DiscordOpcode::PresenceUpdate,
            4 => DiscordOpcode::VoiceStateUpdate,
            6 => DiscordOpcode::Resume,
            7 => DiscordOpcode::Reconnect,
            8 => DiscordOpcode::RequestGuildMembers,
            9 => DiscordOpcode::InvalidSession,
            10 => DiscordOpcode::Hello,
            11 => DiscordOpcode::HeartbeatACK,
            _ => DiscordOpcode::InvalidSession,
        }
    }

    pub fn get_u8(&self) -> u8 {
        match self {
            DiscordOpcode::Dispatch => 0,
            DiscordOpcode::Heartbeat => 1,
            DiscordOpcode::Identify => 2,
            DiscordOpcode::PresenceUpdate => 3,
            DiscordOpcode::VoiceStateUpdate => 4,
            DiscordOpcode::Resume => 6,
            DiscordOpcode::Reconnect => 7,
            DiscordOpcode::RequestGuildMembers => 8,
            DiscordOpcode::InvalidSession => 9,
            DiscordOpcode::Hello => 1,
            DiscordOpcode::HeartbeatACK => 1,
        }
    }
}

pub fn handle_heartbeat(frame: WSFrame, seq: Arc<Mutex<AtomicU32>>, tx: Sender<WSFrame>) {
    let heartbeat_data: GatewayHello = serde_json::from_str(&frame.payload).unwrap();

    // Heartbeats
    thread::spawn(move || loop {
        let sequence = seq.lock().unwrap();
        thread::sleep(time::Duration::from_millis(
            heartbeat_data.d.heartbeat_interval,
        ));
        tx.send(heartbeat_frame(&sequence)).unwrap();
    });
}
