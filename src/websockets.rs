use base64;
use native_tls::TlsConnector;
use rand;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::util::binary_to_decimal;

use self::frame::WSFrame;
use self::opcode::RFC6455Opcode;

pub mod frame;
pub mod opcode;

pub struct WebSocket<'a> {
    uri: &'a str,
}

impl<'a> WebSocket<'a> {
    pub fn new(uri: &'a str) -> Self {
        WSFrame::new(
            true,
            true,
            RFC6455Opcode::Text,
            20000,
            String::from("{lesgo: or not}"),
        )
        .encode();

        WebSocket { uri }
    }

    pub fn initialize(&mut self) -> (Sender<WSFrame>, Receiver<WSFrame>) {
        let connector = TlsConnector::new().unwrap();
        let tcp_stream = TcpStream::connect(format!("{}:443", &self.uri)).unwrap();
        let mut stream = connector.connect(&self.uri, tcp_stream).unwrap();

        let upgrade_request_string = create_upgrade_string(&self.uri);
        stream.write(upgrade_request_string.as_bytes()).unwrap();

        // Send to main with sender and they receive through receiver
        let (recv_sender, recv_receiver) = channel::<WSFrame>();
        let (send_sender, send_receiver) = channel::<WSFrame>();
        thread::spawn(move || {
            loop {
                let mut buf = [0; 8192];
                stream.read(&mut buf).unwrap();

                if String::from_utf8_lossy(&buf).contains("101 Switching Protocols") {
                    // do more stuff prob, upgrade succesful
                    continue;
                }

                let frame: WSFrame = Self::frame_parser(buf.to_vec());
                recv_sender.send(frame).unwrap();

                let mut request = send_receiver.recv().unwrap();
                stream.write(&request.encode()).unwrap();
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
        let payload = String::from_utf8_lossy(&frame[2..]);

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
