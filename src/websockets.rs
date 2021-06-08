use base64;
use rand;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

use crate::util::binary_to_decimal;

use self::frame::WSFrame;
use self::opcode::RFC6455Opcode;
use self::socket::Socket;

pub mod frame;
pub mod opcode;
pub mod socket;

pub struct WebSocket<'a> {
    uri: &'a str,
}

impl<'a> WebSocket<'a> {
    pub fn new(uri: &'a str) -> Self {
        WebSocket { uri }
    }

    pub fn initialize(&mut self) -> (Sender<WSFrame>, Receiver<WSFrame>) {
        let socket = Arc::new(Socket::bind(&self.uri, 443, true).unwrap());
        let socket_read = Arc::clone(&socket);
        let socket_write = Arc::clone(&socket);

        let upgrade_request_string = create_upgrade_string(&self.uri);
        let mut output_stream = socket.output_stream.lock().unwrap();
        output_stream
            .write(upgrade_request_string.as_bytes())
            .unwrap();

        let (recv_sender, recv_receiver) = channel::<WSFrame>();
        thread::spawn(move || {
            let mut input_stream = socket_read.input_stream.lock().unwrap();
            loop {
                let mut buf = [0; 8192];
                input_stream.read(&mut buf).unwrap();

                if String::from_utf8_lossy(&buf).contains("101 Switching Protocols") {
                    // do more stuff prob, upgrade succesful
                    continue;
                }

                let frame: WSFrame = Self::frame_parser(buf.to_vec());
                recv_sender.send(frame).unwrap();
            }
        });

        let (send_sender, send_receiver) = channel::<WSFrame>();
        thread::spawn(move || {
            let mut output_stream = socket_write.output_stream.lock().unwrap();
            loop {
                let mut request = send_receiver.recv().unwrap();
                output_stream.write(&request.encode()).unwrap();
            }
        });

        return (send_sender, recv_receiver);
    }

    fn frame_parser(mut frame: Vec<u8>) -> WSFrame {
        if (frame.first().unwrap() & 0x70) != 0x00 {
            // throw error stuff as there seems to be some error
        }

        let fin = (frame.first().unwrap() & 0x80) == 0x80;
        let mask = (frame.get(1).unwrap() & 0x80) == 0x80;
        let opcode: u8 = frame.first().unwrap() & 0x0F;
        let mut payload_length: usize = (frame.get(1).unwrap() & 0x7F).into();
        let mut pre_payload_length: usize = 2;

        if payload_length > 125 {
            pre_payload_length = match payload_length {
                126 => 4,
                127 => 10,
                _ => 4,
            };

            let decimal = frame.get(2..pre_payload_length).unwrap();
            let binary: Vec<String> = decimal.iter().map(|d| format!("{:8b}", d)).collect();
            payload_length = binary_to_decimal(binary.concat().as_str()) as usize;
        }

        frame.drain(payload_length + pre_payload_length..frame.len());
        let payload = String::from_utf8_lossy(&frame[pre_payload_length..]);

        WSFrame::new(
            fin,
            mask,
            RFC6455Opcode::from_u8(opcode),
            payload_length as usize,
            payload.to_string(),
        )
    }
}

fn create_upgrade_string(uri: &str) -> String {
    let random_key: [u8; 16] = rand::random();
    let encoded_key: String = base64::encode(&random_key);

    let upgrade_request_literal = [
        "GET /?v=9 HTTP/1.1",
        format!("Host: {}", &uri).as_str(),
        "Authorization: Bot",
        "Connection: Upgrade",
        "Upgrade: websocket",
        "Sec-WebSocket-Version: 13",
        format!("Sec-WebSocket-Key: {}", &encoded_key).as_str(),
        "\r\n",
    ]
    .join("\r\n");

    String::from(upgrade_request_literal)
}
