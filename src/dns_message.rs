#![allow(dead_code)]
use crate::dns_error::DNSError;
use modular_bitfield::prelude::*;
use std::net::Ipv4Addr;
use std::result::Result;

// TODO: handle error
//      * when splitting => should return error if domain not valid
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

#[bitfield]
#[derive(Clone)]
pub struct HeaderFlags {
    // First byte
    pub rd: B1,
    pub tc: B1,
    pub aa: B1,
    pub opcode: B4,
    pub qr: B1,
    // Second Byte
    pub rcode: B4,
    #[skip(getters, setters)]
    pub reserved: B3,
    pub ra: B1,
}

pub struct Header {
    pub id: u16,
    pub flags: HeaderFlags,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl Header {
    pub fn new(
        id: u16,
        flags: HeaderFlags,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
    ) -> Self {
        Self {
            id: id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    pub fn from_bytes(source: &[u8]) -> Self {
        // The following should never crash, source is at least 12 bytes long
        assert_eq!(source.len(), 12);

        let id = u16::from_be_bytes(source[0..2].try_into().expect("Error parsing flags"));
        let flags = HeaderFlags::from_bytes(source[2..4].try_into().expect("Error parsing flags"));
        let qdcount = u16::from_be_bytes(source[4..6].try_into().expect("Error parsing flags"));
        let ancount = u16::from_be_bytes(source[6..8].try_into().expect("Error parsing flags"));
        let nscount = u16::from_be_bytes(source[8..10].try_into().expect("Error parsing flags"));
        let arcount = u16::from_be_bytes(source[10..12].try_into().expect("Error parsing flags"));

        Self {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = vec![];

        response.extend_from_slice(&self.id.to_be_bytes());
        response.extend_from_slice(&self.flags.clone().into_bytes());
        response.extend_from_slice(&self.qdcount.to_be_bytes());
        response.extend_from_slice(&self.ancount.to_be_bytes());
        response.extend_from_slice(&self.nscount.to_be_bytes());
        response.extend_from_slice(&self.arcount.to_be_bytes());

        response
    }
}

struct Question {
    labels: Vec<(u8, String)>,
    question_type: u16,
    class: u16,
}

impl Question {
    pub fn new(domain: String, question_type: u16, class: u16) -> Self {
        Self {
            labels: get_labels(domain),
            question_type,
            class,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut question = vec![];
        for (len, label) in self.labels.iter() {
            question.push(*len);
            question.extend_from_slice(&label.clone().into_bytes())
        }
        // Push null to terminate label sequence
        question.push(0);
        question.extend_from_slice(&self.question_type.to_be_bytes());
        question.extend_from_slice(&self.class.to_be_bytes());

        return question;
    }
}

struct Answer {
    labels: Vec<(u8, String)>,
    answer_type: u16,
    class: u16,
    ttl: u32,
    len: u16,
    data: Ipv4Addr,
}

impl Answer {
    pub fn new(domain: String, answer_type: u16, class: u16, ttl: u32, ip: Ipv4Addr) -> Self {
        Self {
            labels: get_labels(domain),
            answer_type,
            class,
            ttl,
            len: 0,
            data: ip,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut answer = vec![];

        answer.extend_from_slice(&labels_to_bytes(&self.labels));
        // Push null to terminate label sequence
        answer.push(0);
        answer.extend_from_slice(&self.answer_type.to_be_bytes());
        answer.extend_from_slice(&self.class.to_be_bytes());
        answer.extend_from_slice(&self.ttl.to_be_bytes());
        answer.extend_from_slice(&self.len.to_be_bytes());
        let ip_octets = self.data.octets();
        answer.extend_from_slice(&ip_octets.len().to_be_bytes());
        answer.extend_from_slice(&ip_octets);

        return answer;
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
