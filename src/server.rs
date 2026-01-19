use crate::dns_error::DNSError;
use crate::dns_message::answer::Answer;
use crate::dns_message::{DNSMessage, header::HeaderFlags};
use std::io::Result;
use std::net::Ipv4Addr;
use std::net::UdpSocket;

const TTL: u32 = 60;

pub struct Server {
    listen_ip: Ipv4Addr,
    port: u16,
    follow_server: Option<Ipv4Addr>,
}

impl Server {
    pub fn new(listen_ip: Ipv4Addr, port: u16, follow_server: Option<Ipv4Addr>) -> Self {
        Self {
            listen_ip,
            port,
            follow_server,
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let udp_socket = UdpSocket::bind((self.listen_ip, self.port))?;
        let mut buf = [0; 512];
        loop {
            match udp_socket.recv_from(&mut buf) {
                Ok((size, source)) => {
                    println!("Received {} bytes from {}", size, source);
                    match self.get_request(size, buf) {
                        Ok(request) => {
                            let response = self.create_response(request);
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
        Ok(())
    }

    fn create_response(&self, request: DNSMessage) -> Vec<u8> {
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

        let mut questions = vec![];
        let mut answers = vec![];
        for question in request.questions {
            questions.push(question.clone());
            answers.push(Answer::new(
                question.get_domain(),
                1,
                1,
                TTL,
                Ipv4Addr::new(8, 8, 8, 8),
            ));
        }

        let response = DNSMessage::new(request.header.id, flags, questions, answers);

        return response.to_bytes();
    }

    fn get_request(&self, size: usize, buffer: [u8; 512]) -> Result<DNSMessage, DNSError> {
        Ok(DNSMessage::from_buffer(size, &buffer)?)
    }
}
