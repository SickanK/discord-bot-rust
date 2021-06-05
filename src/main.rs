pub mod websockets;
use websockets::WebSocket;

//use self::websockets::get_length;

fn main() {
    WebSocket::new("gateway.discord.gg");

    loop {}
}
