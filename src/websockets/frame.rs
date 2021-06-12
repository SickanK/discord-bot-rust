use super::opcode::RFC6455Opcode;
use rand;

#[derive(Debug, Clone)]
pub struct WSFrame {
    pub fin: bool,
    pub mask: bool,
    pub opcode: RFC6455Opcode,
    pub payload_length: usize,
    pub payload: String,
}

impl WSFrame {
    pub fn new(
        fin: bool,
        mask: bool,
        opcode: RFC6455Opcode,
        payload_length: usize,
        payload: String,
    ) -> Self {
        WSFrame {
            fin,
            mask,
            opcode,
            payload_length,
            payload,
        }
    }

    pub fn encode(&mut self) -> Vec<u8> {
        let mut req = Vec::new();

        let mut first_byte: u8 = 0b0000_0000;
        if self.fin {
            first_byte |= 0b1000_0000;
        }

        // ... rsv1, rsv2, rsv3 ...

        first_byte |= &self.opcode.get_u8();

        let mut second_byte: u8 = 0b0000_0000;

        if self.mask {
            second_byte |= 0b1000_0000;
        }

        req.push(first_byte);

        if self.payload_length < 126 {
            second_byte |= self.payload_length as u8;
            req.push(second_byte);
        } else {
            let mut shift = match self.payload_length {
                0..=65535 => 16,
                _ => 64,
            };

            let mut bytes: Vec<u8> = Vec::new();
            let byte_len = match self.payload_length {
                0..=65535 => 2,
                _ => 8,
            };

            if byte_len == 2 {
                second_byte |= 126;
            } else {
                second_byte |= 127;
            }

            for _ in 0..byte_len {
                shift -= 8;
                bytes.push(((self.payload_length >> shift) & 0xff) as u8);
            }

            req.push(second_byte);
            req.append(&mut bytes);
        }

        let mask: [u8; 4] = rand::random();
        req.append(&mut mask.to_vec());

        let payload_buffer = self.payload.as_bytes();
        let mut payload_buffer = mask_data(mask, &payload_buffer);
        req.append(&mut payload_buffer);

        req
    }
}

fn mask_data(mask: [u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let zip_iter = data.iter().zip(mask.iter().cycle());
    for (&buf_item, &key_item) in zip_iter {
        out.push(buf_item ^ key_item);
    }
    out
}
