use crate::labels_helpers::{labels_from_string, labels_from_bytes, labels_to_bytes};
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct Answer {
    labels: Vec<(u8, String)>,
    answer_type: u16,
    class: u16,
    ttl: u32,
    rdlength: u16,
    data: Ipv4Addr,
    pub len: usize,
}

impl Answer {
    pub fn new(domain: String, answer_type: u16, class: u16, ttl: u32, ip: Ipv4Addr) -> Self {
        let ip_octets = ip.octets();
        Self {
            labels: labels_from_string(domain),
            answer_type,
            class,
            ttl,
            rdlength: ip_octets.len() as u16,
            data: ip,
            len: 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut answer = vec![];

        answer.extend_from_slice(&labels_to_bytes(&self.labels));
        // Push null to terminate label sequence
        answer.push(0);
        answer.extend_from_slice(&self.answer_type.to_be_bytes());
        answer.extend_from_slice(&self.class.to_be_bytes());
        println!("len: {}", self.ttl);
        answer.extend_from_slice(&self.ttl.to_be_bytes());
        answer.extend_from_slice(&self.rdlength.to_be_bytes());
        let ip_octets = self.data.octets();
        answer.extend_from_slice(&ip_octets);

        return answer;
    }

    pub fn from_bytes(buffer: &[u8], offset: usize) -> Self {
        let (labels, size) = labels_from_bytes(buffer, offset);
        let offset = offset + size;
        if offset + 14 > buffer.len() {
            todo!("Handle size error")
        }

        // TODO: refactor, finder more idiomatic way
        let answer_type = u16::from_be_bytes(buffer[offset..offset + 2].try_into().unwrap());
        let class = u16::from_be_bytes(buffer[offset + 2..offset + 4].try_into().unwrap());
        let ttl = u32::from_be_bytes(buffer[offset + 4..offset + 8].try_into().unwrap());
        let rdlength = u16::from_be_bytes(buffer[offset + 8..offset + 10].try_into().unwrap());
        let rdata_offset = offset + 10 as usize;
        let data = Ipv4Addr::new(
            buffer[rdata_offset],
            buffer[rdata_offset + 1],
            buffer[rdata_offset + 2],
            buffer[rdata_offset + 3],
        );
        let len = size + 10 + 4;

        Self {
            labels,
            answer_type,
            class,
            ttl,
            rdlength,
            data,
            len,
        }
    }
}
