use crate::dns_message::DNSMessage;
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
                println!("Printing bytes: {}", response.len());
                println!("{:?} ", response);
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

fn create_response() -> [u8; 12] {
    let response = DNSMessage::new();

    return response.to_bytes();
}
