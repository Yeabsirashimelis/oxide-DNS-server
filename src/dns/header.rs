#[derive(Debug, Clone)]
pub struct DnsHeader {
    pub id: u16,
    pub qr: u8,
    pub opcode: u8,
    pub aa: u8,
    pub tc: u8,
    pub rd: u8,
    pub ra: u8,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn parse(data: &[u8]) -> Self {
        Self {
            id: u16::from_be_bytes([data[0], data[1]]),
            qr: (data[2] >> 7) & 1,
            opcode: (data[2] >> 3) & 0b1111,
            aa: (data[2] >> 2) & 1,
            tc: (data[2] >> 1) & 1,
            rd: data[2] & 1,
            ra: (data[3] >> 7) & 1,
            z: (data[3] >> 4) & 0b111,
            rcode: data[3] & 0b1111,
            qdcount: u16::from_be_bytes([data[4], data[5]]),
            ancount: u16::from_be_bytes([data[6], data[7]]),
            nscount: u16::from_be_bytes([data[8], data[9]]),
            arcount: u16::from_be_bytes([data[10], data[11]]),
        }
    }
}
