#[derive(Debug, Clone)]
pub enum RFC6455Opcode {
    Continuation,
    Text,
    Binary,
    Close,
    Ping,
    Pong,
}

impl RFC6455Opcode {
    pub fn from_u8(code: u8) -> Self {
        match code {
            0x0 => RFC6455Opcode::Continuation,
            0x1 => RFC6455Opcode::Text,
            0x2 => RFC6455Opcode::Binary,
            0x8 => RFC6455Opcode::Close,
            0x9 => RFC6455Opcode::Ping,
            0xA => RFC6455Opcode::Pong,
            _ => RFC6455Opcode::Close,
        }
    }

    pub fn get_u8(&self) -> u8 {
        match self {
            RFC6455Opcode::Continuation => 0x0,
            RFC6455Opcode::Text => 0x1,
            RFC6455Opcode::Binary => 0x2,
            RFC6455Opcode::Close => 0x8,
            RFC6455Opcode::Ping => 0x9,
            RFC6455Opcode::Pong => 0xA,
        }
    }
}
