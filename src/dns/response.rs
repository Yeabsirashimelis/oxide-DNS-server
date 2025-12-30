use super::packet::DnsPacket;

pub fn build_response(request: &DnsPacket) -> Vec<u8> {
    let mut response = vec![0u8; 12];

    response[0..2].copy_from_slice(&request.header.id.to_be_bytes());

    // QR = 1 (response), opcode = 0, RD copied
    response[2] = 0b1000_0000;
    response[3] = 0;

    // counts
    response[4..6].copy_from_slice(&0u16.to_be_bytes());
    response[6..8].copy_from_slice(&0u16.to_be_bytes());
    response[8..10].copy_from_slice(&0u16.to_be_bytes());
    response[10..12].copy_from_slice(&0u16.to_be_bytes());

    response
}
