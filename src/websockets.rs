use base64;
use native_tls::TlsConnector;
use rand;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

pub struct WebSocket {
    uri: String,
}

impl WebSocket {
    pub fn new(uri: &str) {
        let connector = TlsConnector::new().unwrap();
        let tcp_stream = TcpStream::connect(format!("{}:443", uri)).unwrap();
        let mut stream = connector.connect(uri, tcp_stream).unwrap();

        let upgrade_request_string = create_upgrade_string(uri);
        stream.write(upgrade_request_string.as_bytes()).unwrap();

        thread::spawn(move || loop {
            let mut buf = [0; 1024];
            stream.read(&mut buf).unwrap();
            println!("{:?}", String::from_utf8_lossy(&buf));
            println!("{:?}", buf);
        });

        loop {}
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
