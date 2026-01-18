use crate::dns_message::{DNSMessage, HeaderFlags};
#[allow(unused_imports)]
use std::net::UdpSocket;

pub mod dns_message;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = create_response();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn create_response() -> Vec<u8> {
    let flags = HeaderFlags::new()
        .with_qr(1)
        .with_opcode(0)
        .with_aa(0)
        .with_tc(0)
        .with_rd(0)
        .with_ra(0)
        .with_rcode(0);

    let response = DNSMessage::new(flags, 1, 0, 0, 0, "codecrafters.io".to_string());

    return response.to_bytes();
}
