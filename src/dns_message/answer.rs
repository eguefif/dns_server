use crate::dns_message::{get_labels, labels_to_bytes};
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Answer {
    labels: Vec<(u8, String)>,
    answer_type: u16,
    class: u16,
    ttl: u32,
    len: u16,
    data: Ipv4Addr,
}

impl Answer {
    pub fn new(domain: String, answer_type: u16, class: u16, ttl: u32, ip: Ipv4Addr) -> Self {
        let ip_octets = ip.octets();
        Self {
            labels: get_labels(domain),
            answer_type,
            class,
            ttl,
            len: ip_octets.len() as u16,
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
        answer.extend_from_slice(&ip_octets);

        return answer;
    }
}
