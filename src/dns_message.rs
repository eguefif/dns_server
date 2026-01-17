#![allow(dead_code)]
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Clone)]
struct HeaderFlags {
    // First byte
    rd: B1,
    tc: B1,
    aa: B1,
    opcode: B4,
    qr: B1,
    // Second Byte
    rcode: B4,
    reserved: B3,
    ra: B1,
}

struct DNSHeader {
    id: u16,
    flags: HeaderFlags,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DNSHeader {
    pub fn new() -> Self {
        let flags = HeaderFlags::new()
            .with_qr(0b1)
            .with_opcode(0)
            .with_aa(0)
            .with_tc(0)
            .with_rd(0)
            .with_ra(0)
            .with_reserved(0)
            .with_rcode(0);

        Self {
            id: 1234,
            flags,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut response = [0; 12];

        let id = self.id.to_be_bytes();
        response[0] = id[0];
        response[1] = id[1];

        let flags = self.flags.clone().into_bytes();
        println!("{:b} {:b}", flags[0], flags[1]);
        response[2] = flags[0];
        response[3] = flags[1];

        let qdcount = self.qdcount.to_be_bytes();
        response[4] = qdcount[0];
        response[5] = qdcount[1];

        let ancount = self.ancount.to_be_bytes();
        response[6] = ancount[0];
        response[7] = ancount[1];

        let nscount = self.nscount.to_be_bytes();
        response[8] = nscount[0];
        response[9] = nscount[1];

        let arcount = self.arcount.to_be_bytes();
        response[10] = arcount[0];
        response[11] = arcount[1];

        response
    }
}

pub struct DNSMessage {
    header: DNSHeader,
}

impl DNSMessage {
    pub fn new() -> Self {
        let header = DNSHeader::new();
        Self { header }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        return self.header.to_bytes();
    }
}
