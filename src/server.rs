use crate::dns_message::answer::Answer;
use crate::dns_message::header::Header;
use crate::dns_message::question::Question;
use crate::dns_message::{DNSMessage, header::HeaderFlags};
use std::error::Error;
use std::net::UdpSocket;
use std::net::{Ipv4Addr, SocketAddr};
use std::result::Result;

const UDP_MAX: usize = 65535;

pub struct Server {
    follow_server: SocketAddr,
    udp_socket: UdpSocket,
}

impl Server {
    pub fn new(
        listen_ip: Ipv4Addr,
        port: u16,
        follow_server: SocketAddr,
    ) -> Result<Self, Box<dyn Error>> {
        let udp_socket = UdpSocket::bind((listen_ip, port))?;
        Ok(Self {
            follow_server,
            udp_socket,
        })
    }

    pub fn run(&self) -> std::result::Result<(), Box<dyn Error>> {
        let mut buffer = [0; UDP_MAX];
        loop {
            match self.udp_socket.recv_from(&mut buffer) {
                Ok((size, source)) => {
                    println!("Received {} bytes from {}", size, source);
                    match DNSMessage::from_buffer(size, &buffer) {
                        Ok(request) => {
                            let (questions, answers, header) = self.get_follow_response(request)?;
                            let response = self.create_response(header, questions, answers);
                            self.udp_socket
                                .send_to(&response.to_bytes(), source)
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

    fn get_follow_response(
        &self,
        request: DNSMessage,
    ) -> Result<(Vec<Question>, Vec<Answer>, Header), Box<dyn Error>> {
        let mut buf = [0; UDP_MAX];

        let questions = request.questions;
        let header = request.header;
        let mut follow_questions = vec![];
        let mut follow_answers = vec![];
        for request_question in questions.into_iter() {
            let mut follow_request = DNSMessage::from_request_header(&header, 1, 0);

            follow_request.questions.push(request_question);
            self.udp_socket
                .send_to(&follow_request.to_bytes(), self.follow_server)?;


            let (size, _) = self.udp_socket.recv_from(&mut buf)?;
            let mut follow_response = DNSMessage::from_buffer(size, &buf)?;
            if let Some(question) = follow_response.questions.pop() {
                if let Some(answer) = follow_response.answers.pop() {
                    follow_questions.push(question);
                    follow_answers.push(answer);
                }
            }
        }

        Ok((follow_questions, follow_answers, header))
    }

    fn create_response(
        &self,
        header: Header,
        questions: Vec<Question>,
        answers: Vec<Answer>,
    ) -> DNSMessage {
        let rcode = if header.flags.opcode() == 0 {
            0
        } else {
            4
        };
        let flags = HeaderFlags::new()
            .with_qr(1)
            .with_opcode(header.flags.opcode())
            .with_aa(0)
            .with_tc(0)
            .with_rd(header.flags.rd())
            .with_ra(0)
            .with_rcode(rcode);

        let mut response_questions = vec![];
        let mut response_answers = vec![];
        for question in questions.into_iter() {
            response_questions.push(question);
        }
        for answer in answers.into_iter() {
            response_answers.push(answer);
        }

        let response = DNSMessage::new(
            header.id,
            flags,
            response_questions,
            response_answers,
        );

        response
    }
}
