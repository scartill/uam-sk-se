use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
    socket.send_to(&[0; 10], "127.0.0.1:4242").expect("couldn't send data");
}
