use std::net::{ UdpSocket};
use crate::dns::packet::DnsPacket;
use crate::dns::response::build_response;

pub fn start(addr: &str) {
    let socket = UdpSocket::bind(addr).expect("Failed to bind");
    let mut buf = [0u8; 512];

    loop {
        let (size, source) = socket.recv_from(&mut buf).expect("Receive failed");

        let request = DnsPacket::parse(&buf[..size]);
        let response = build_response(&request);

        socket.send_to(&response, source).expect("Send failed");
    }
}
