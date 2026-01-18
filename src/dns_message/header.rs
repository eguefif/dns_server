use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub flags: HeaderFlags,
    pub qdcount: u16,
    pub ancount: u16,
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
