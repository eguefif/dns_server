#![allow(dead_code)]
use modular_bitfield::prelude::*;

pub struct DNSMessage {
    header: Header,
    question: Question,
}

impl DNSMessage {
    pub fn new(
        flags: HeaderFlags,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
        domain: String,
    ) -> Self {
        let header = Header::new(flags, qdcount, ancount, nscount, arcount);
        let question = Question::new(domain, 1, 1);

        Self { header, question }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.question.to_bytes());
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

struct Header {
    id: u16,
    flags: HeaderFlags,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl Header {
    pub fn new(flags: HeaderFlags, qdcount: u16, ancount: u16, nscount: u16, arcount: u16) -> Self {
        Self {
            id: 1234,
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
        let splits = domain.split(".");
        let mut labels = vec![];
        for split in splits {
            labels.push((split.len() as u8, split.to_string()));
        }
        Self {
            labels,
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
