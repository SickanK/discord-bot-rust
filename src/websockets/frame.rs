use super::opcode::RFC6455Opcode;
use rand;

#[derive(Debug)]
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

            for _ in 0..byte_len {
                shift -= 8;
                bytes.push(((self.payload_length >> shift) & 0xff) as u8);
            }

            println!("{:?}", bytes);
            req.push(second_byte);
            req.append(&mut bytes);
        }

        let mask: [u8; 4] = rand::random();
        req.append(&mut mask.to_vec());

        let payload_buffer = self.payload.as_bytes();
        let mut payload_buffer = payload_buffer.to_vec();
        apply_mask(&mut payload_buffer, mask);
        req.append(&mut payload_buffer);

        req
    }
}

pub fn apply_mask(buf: &mut [u8], mask: [u8; 4]) {
    apply_mask_fast32(buf, mask)
}

fn apply_mask_fallback(buf: &mut [u8], mask: [u8; 4]) {
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte ^= mask[i & 3];
    }
}

pub fn apply_mask_fast32(buf: &mut [u8], mask: [u8; 4]) {
    let mask_u32 = u32::from_ne_bytes(mask);

    let (mut prefix, words, mut suffix) = unsafe { buf.align_to_mut::<u32>() };
    apply_mask_fallback(&mut prefix, mask);
    let head = prefix.len() & 3;
    let mask_u32 = if head > 0 {
        if cfg!(target_endian = "big") {
            mask_u32.rotate_left(8 * head as u32)
        } else {
            mask_u32.rotate_right(8 * head as u32)
        }
    } else {
        mask_u32
    };
    for word in words.iter_mut() {
        *word ^= mask_u32;
    }
    apply_mask_fallback(&mut suffix, mask_u32.to_ne_bytes());
}
