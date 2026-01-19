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

    // This constructor returns a DNSMessage with the header passed
    // in argument, set the right rcode and the qr bit.
    // Questions and answers are initialized with empty vectors
    pub fn from_request_header(request: &DNSMessage, qdcount: u16, ancount: u16) -> Self {
        let flags = HeaderFlags::new()
            .with_qr(0)
            .with_opcode(request.header.flags.opcode())
            .with_aa(0)
            .with_tc(0)
            .with_rd(request.header.flags.rd())
            .with_ra(0)
            .with_rcode(request.header.flags.rcode());
        let header = Header::new(request.header.id, flags, qdcount, ancount, 0, 0);
        let questions = vec![];
        let answers = vec![];
        Self {
            header,
            questions,
            answers,
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

fn get_labels(domain: String) -> Labels {
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

type Labels = Vec<(u8, String)>;

fn labels_from_bytes(buffer: &[u8], mut offset: usize) -> (Labels, usize) {
    let mut labels = vec![];

    let mut iter = buffer[offset..].iter().peekable();
    let mut labels_size = 0;
    let mut compression = false;
    loop {
        let Some(size) = iter.next() else {
            todo!("handle error: early stop")
        };
        if *size == 0b1100_0000 {
            offset = *iter.next().unwrap() as usize;
            iter = buffer[offset..].iter().peekable();
            labels_size += 2;
            compression = true;
            continue;
        }
        if !compression {
            labels_size += *size as usize + 1;
        }
        let mut label = String::new();
        for _ in 0..*size {
            let byte = iter.next().unwrap();
            label.push(*byte as char);
        }
        labels.push((*size, label));
        let Some(&peek) = iter.peek() else {
            todo!("Handle error: early stop")
        };
        if *peek == 0 {
            if !compression {
                labels_size += 1;
            }
            break;
        }
    }

    (labels, labels_size)
}

fn get_labels_size(labels: &Labels) -> usize {
    let mut size: usize = 0;
    for (s, _) in labels {
        size += *s as usize + 1
    }

    size as usize + 1
}
