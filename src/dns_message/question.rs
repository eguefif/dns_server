use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use crate::dns_error::DNSError;
use crate::labels_helpers::{Labels, labels_from_bytes, labels_from_string};

#[derive(Debug, Clone)]
pub struct Question {
    labels: Labels,
    question_type: u16,
    class: u16,
    pub len: usize,
}

impl Question {
    pub fn new(domain: String, question_type: u16, class: u16) -> Self {
        Self {
            labels: labels_from_string(domain),
            question_type,
            class,
            len: 0,
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

    pub fn from_bytes(buffer: &[u8], offset: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let (labels, size) = labels_from_bytes(buffer, offset)?;
        let offset = offset + size;
        if offset + 4 > buffer.len() {
            return Err(Box::new(DNSError::QuestionSizeError));
        }

        let mut cursor = Cursor::new(&buffer[offset..]);
        let question_type = cursor.read_u16::<BigEndian>()?;
        let class = cursor.read_u16::<BigEndian>()?;

        Ok(Self {
            labels,
            question_type,
            class,
            len: size + 4,
        })
    }

    pub fn get_domain(&self) -> String {
        let mut domain = String::new();
        let mut peek = self.labels.iter().peekable();

        loop {
            let Some((_, label)) = peek.next() else {
                todo!("Handle first next that has no label. It means its empty")
            };
            domain.push_str(label);
            let Some(_) = peek.peek() else {
                break;
            };
            domain.push('.');
        }
        domain
    }
}
