use super::header::DnsHeader;

pub struct DnsPacket {
    pub header: DnsHeader,
    // later:
    // pub questions: Vec<DnsQuestion>
}

impl DnsPacket {
    pub fn parse(data: &[u8]) -> Self {
        let header = DnsHeader::parse(data);
        Self { header }
    }
}
