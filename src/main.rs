use std::net::SocketAddr;
#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let request = parse_and_build_request(&buf);
                let response = build_response(request);
                return_response(&udp_socket, &response, source);
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

struct Request {
    packet_identifier: u16,
    response_indicator: u8,
    operation_code: u8,
    authorative_answer: u8,
    truncation: u8,
    recursion_desired: u8,
    recursion_available: u8,
    reserved: u8,
    response_code: u8,
    question_count: u16,
    answer_record_count: u16,
    authorative_record_count: u16,
    additional_record_count: u16,
}

impl Request {
    fn new(
        packet_identifier: u16,
        response_indicator: u8,
        operation_code: u8,
        authorative_answer: u8,
        truncation: u8,
        recursion_desired: u8,
        recursion_available: u8,
        reserved: u8,
        response_code: u8,
        question_count: u16,
        answer_record_count: u16,
        authorative_record_count: u16,
        additional_record_count: u16,
    ) -> Self {
        Self {
            packet_identifier,
            response_indicator,
            operation_code,
            authorative_answer,
            truncation,
            recursion_desired,
            recursion_available,
            reserved,
            response_code,
            question_count,
            answer_record_count,
            authorative_record_count,
            additional_record_count,
        }
    }
}

fn parse_and_build_request(data: &[u8]) -> Request {
    let packet_identifier = u16::from_be_bytes([data[0], data[1]]);
    let response_indicator = (data[2] >> 7) & 1;
    let operation_code = (data[2] >> 3) & 0b1111;
    let authorative_answer = (data[2] >> 2) & 1;
    let truncation = (data[2] >> 1) & 1;
    let recursion_desired = data[2] & 1;
    let recursion_available = (data[3] >> 7) & 1;
    let reserved = (data[3] >> 4) & 0b111;
    let response_code = (data[3]) & 0b1111;
    let question_count = u16::from_be_bytes([data[4], data[5]]);
    let answer_record_count = u16::from_be_bytes([data[6], data[7]]);
    let authorative_record_count = u16::from_be_bytes([data[8], data[9]]);
    let additional_record_count = u16::from_be_bytes([data[10], data[11]]);

    let request = Request::new(
        packet_identifier,
        response_indicator,
        operation_code,
        authorative_answer,
        truncation,
        recursion_desired,
        recursion_available,
        reserved,
        response_code,
        question_count,
        answer_record_count,
        authorative_record_count,
        additional_record_count,
    );

    request
}

fn build_response(request: Request) -> Vec<u8> {
    let mut response = [0u8; 12];

    response[0..2].copy_from_slice(&request.packet_identifier.to_be_bytes());

    response[2..4].copy_from_slice(&0x8000u16.to_be_bytes());

    response[4..6].copy_from_slice(&0u16.to_be_bytes());
    response[6..8].copy_from_slice(&0u16.to_be_bytes());
    response[8..10].copy_from_slice(&0u16.to_be_bytes());
    response[10..12].copy_from_slice(&0u16.to_be_bytes());

    response.to_vec()
}

fn return_response(socket: &UdpSocket, response: &[u8], source: SocketAddr) {
    socket
        .send_to(response, source)
        .expect("Failed to send response");
}
