#![allow(dead_code)]
use crate::dns_error::DNSError;
use crate::dns_message::answer::Answer;
use crate::dns_message::header::{Header, HeaderFlags};
use crate::dns_message::question::Question;
use std::net::Ipv4Addr;
use std::result::Result;

pub mod answer;
pub mod header;
pub mod question;

pub struct DNSMessage {
    pub header: Header,
    question: Question,
    answer: Answer,
}

impl DNSMessage {
    pub fn new(
        id: u16,
        flags: HeaderFlags,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
        domain: String,
        question_type: u16,
        question_class: u16,
        answer_type: u16,
        answer_class: u16,
        ttl: u32,
        ip: Ipv4Addr,
    ) -> Self {
        let header = Header::new(id, flags, qdcount, ancount, nscount, arcount);
        let question = Question::new(domain.clone(), question_type, question_class);
        let answer = Answer::new(domain.clone(), answer_type, answer_class, ttl, ip);

        Self {
            header,
            question,
            answer,
        }
    }

    pub fn from_buffer(size: usize, source: &[u8]) -> Result<Self, DNSError> {
        if size < 13 {
            return Err(DNSError::RequestHeaderSizeError(size));
        }
        let header = Header::from_bytes(&source[0..12]);
        let question = Question::new("codecrafters.io".to_string(), 1, 1);
        let answer = Answer::new(
            "codecrafters.io".to_string(),
            1,
            1,
            60,
            Ipv4Addr::new(8, 8, 8, 8),
        );

        Ok(Self {
            header,
            question,
            answer,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.question.to_bytes());
        bytes.extend_from_slice(&self.answer.to_bytes());
        return bytes;
    }
}

fn get_labels(domain: String) -> Vec<(u8, String)> {
    let splits = domain.split(".");
    let mut labels = vec![];
    for split in splits {
        labels.push((split.len() as u8, split.to_string()));
    }

    labels
}

fn labels_to_bytes(labels: &[(u8, String)]) -> Vec<u8> {
    let mut bytes = vec![];

    for (len, label) in labels {
        bytes.push(*len);
        bytes.extend_from_slice(&label.clone().into_bytes());
    }

    return bytes;
}
