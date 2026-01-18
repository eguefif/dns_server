use crate::dns_message::get_labels;

pub(super) struct Question {
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
