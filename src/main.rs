#[allow(unused_imports)]
use std::net::UdpSocket;

struct DnsMessage {
    header: DnsHeader,
    question: DnsQuestion,
}

struct DnsHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

struct DnsQuestion {
    name: Vec<u8>,
    qtype: u16,
    class: u16,
}

fn serialize_dns_message(message: DnsMessage) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(serialize_dns_header(&message.header));
    buffer.extend(serialize_dns_question(&message.question));
    buffer
}

fn serialize_dns_header(header: &DnsHeader) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend(&header.id.to_be_bytes());
    buffer.extend(&header.flags.to_be_bytes());
    buffer.extend(&header.qdcount.to_be_bytes());
    buffer.extend(&header.ancount.to_be_bytes());
    buffer.extend(&header.nscount.to_be_bytes());
    buffer.extend(&header.arcount.to_be_bytes());
    buffer
}

fn serialize_dns_question(question: &DnsQuestion) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend(split_dns_name(&question.name));
    buffer.extend(&question.qtype.to_be_bytes());
    buffer.extend(&question.class.to_be_bytes());
    buffer
}

fn split_dns_name(name: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    for part in name.split(|byte| *byte == b'.') {
        let length = part.len() as u8;
        result.push(length);
        result.extend(part);
    }
    result.push(0);
    result
}

fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let dns_msg = DnsMessage {
                    header: DnsHeader {
                        id: 1234,
                        flags: 0x8000,
                        qdcount: 1,
                        ancount: 0,
                        nscount: 0,
                        arcount: 0,
                    },
                    question: DnsQuestion {
                        name: b"codecrafters.io".to_vec(),
                        qtype: 1,
                        class: 1,
                    },
                };

                let response = serialize_dns_message(dns_msg);
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
