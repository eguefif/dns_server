use crate::dns_message::{Labels, get_labels, labels_from_bytes};

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
            labels: get_labels(domain),
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

    pub fn from_bytes(buffer: &[u8], offset: usize) -> Self {
        let (labels, size) = labels_from_bytes(buffer, offset);
        let qtype_offset = offset + size;
        let class_offset = qtype_offset + 2;
        if qtype_offset + 4 > buffer.len() {
            todo!("Handle size error")
        }

        // TODO: refactor, finder more idiomatic way
        let question_type =
            u16::from_be_bytes(buffer[qtype_offset..qtype_offset + 2].try_into().unwrap());
        let class = u16::from_be_bytes(buffer[class_offset..class_offset + 2].try_into().unwrap());

        Self {
            labels,
            question_type,
            class,
            len: size + 4,
        }
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
