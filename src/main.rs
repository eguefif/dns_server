use crate::dns_error::DNSError;
use crate::dns_message::{DNSMessage, HeaderFlags};
use std::net::Ipv4Addr;
#[allow(unused_imports)]
use std::net::{SocketAddr, UdpSocket};
use std::result::Result;

pub mod dns_error;
pub mod dns_message;

const TTL: u32 = 60;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                match get_request(size, buf) {
                    Ok(request) => {
                        let response = create_response(request);
                        udp_socket
                            .send_to(&response, source)
                            .expect("Failed to send response");
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn create_response(request: DNSMessage) -> Vec<u8> {
    let rcode = if request.header.flags.opcode() == 0 {
        0
    } else {
        4
    };
    let flags = HeaderFlags::new()
        .with_qr(1)
        .with_opcode(request.header.flags.opcode())
        .with_aa(0)
        .with_tc(0)
        .with_rd(request.header.flags.rd())
        .with_ra(0)
        .with_rcode(rcode);

    let ip = Ipv4Addr::new(8, 8, 8, 8);
    let response = DNSMessage::new(
        request.header.id,
        flags,
        1,
        1,
        0,
        0,
        "codecrafters.io".to_string(),
        1,
        1,
        1,
        1,
        TTL,
        ip,
    );

    return response.to_bytes();
}

fn get_request(size: usize, buffer: [u8; 512]) -> Result<DNSMessage, DNSError> {
    Ok(DNSMessage::from_buffer(size, &buffer)?)
}
