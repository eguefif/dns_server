#![allow(dead_code)]
use crate::dns_error::DNSError;
use crate::dns_message::answer::Answer;
use crate::dns_message::header::{Header, HeaderFlags};
use crate::dns_message::question::Question;
use std::result::Result;

pub mod answer;
pub mod header;
pub mod question;

#[derive(Debug)]
pub struct DNSMessage {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl DNSMessage {
    pub fn new(
        id: u16,
        flags: HeaderFlags,
        questions: Vec<Question>,
        answers: Vec<Answer>,
    ) -> Self {
        let header = Header::new(
            id,
            flags,
            questions.len() as u16,
            answers.len() as u16,
            0,
            0,
        );

        Self {
            header,
            questions,
            answers,
        }
    }

    pub fn new_response(
        response_header: &Header,
        questions: Vec<Question>,
        answers: Vec<Answer>,
    ) -> Self {
        let rcode = if response_header.flags.opcode() == 0 {
            0
        } else {
            4
        };
        let flags = HeaderFlags::new()
            .with_qr(1)
            .with_opcode(response_header.flags.opcode())
            .with_aa(0)
            .with_tc(0)
            .with_rd(response_header.flags.rd())
            .with_ra(0)
            .with_rcode(rcode);
        let header = Header::new(
            response_header.id,
            flags,
            questions.len() as u16,
            answers.len() as u16,
            0,
            0,
        );
        Self {
            header,
            questions,
            answers,
        }
    }

    pub fn new_request(
        request_header: &Header,
        questions: Vec<Question>,
    ) -> Self {
        let flags = HeaderFlags::new()
            .with_qr(0)
            .with_opcode(request_header.flags.opcode())
            .with_aa(0)
            .with_tc(0)
            .with_rd(request_header.flags.rd())
            .with_ra(0)
            .with_rcode(request_header.flags.rcode());
        let header = Header::new(
            request_header.id,
            flags,
            questions.len() as u16,
            0,
            0,
            0,
        );
        Self {
            header,
            questions,
            answers: vec![],
        }
    }

    pub fn from_buffer(size: usize, buffer: &[u8]) -> Result<Self, DNSError> {
        if size < 12 {
            return Err(DNSError::RequestHeaderSizeError(size));
        }
        let header = Header::from_bytes(&buffer[0..12]);
        let mut questions = vec![];
        let mut offset = 12;
        for _ in 0..header.qdcount {
            let question = Question::from_bytes(&buffer, offset);
            offset += question.len;
            questions.push(question);
        }

        let mut answers = vec![];

        for _ in 0..header.ancount {
            let answer = Answer::from_bytes(&buffer, offset);
            offset += answer.len;
            answers.push(answer);
        }
        Ok(Self {
            header,
            questions,
            answers,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());
        for question in self.questions.iter() {
            bytes.extend_from_slice(&question.to_bytes());
        }
        for answer in self.answers.iter() {
            bytes.extend_from_slice(&answer.to_bytes());
        }
        return bytes;
    }
}
