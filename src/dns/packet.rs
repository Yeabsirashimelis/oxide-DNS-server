
use super::header::DnsHeader;
use super::question::DnsQuestion;

pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    pub raw_question: Vec<u8>
}

impl DnsPacket {
    pub fn parse(data: &[u8]) -> Self {
        let header = DnsHeader::parse(data);
        let (question, pos, raw_question)=DnsQuestion::parse(data);
        Self { header, question, raw_question }
    }
}
