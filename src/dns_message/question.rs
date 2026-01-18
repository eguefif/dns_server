use crate::dns_message::{Labels, get_labels, get_labels_size, labels_from_bytes};

#[derive(Debug)]
pub struct Question {
    labels: Labels,
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

    pub fn from_bytes(buffer: &[u8]) -> Self {
        let labels = labels_from_bytes(buffer);
        let labels_size = get_labels_size(&labels);
        if labels_size + 4 > buffer.len() {
            todo!("Handle size error")
        }

        let question_type =
            u16::from_be_bytes(buffer[labels_size..labels_size + 2].try_into().unwrap());
        let class =
            u16::from_be_bytes(buffer[labels_size + 2..labels_size + 4].try_into().unwrap());

        Self {
            labels,
            question_type,
            class,
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
