#[allow(unused_imports)]
use std::net::UdpSocket;

struct DnsMessage {
    header: DnsHeader,
    question: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
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

struct DnsAnswer {
    name: Vec<u8>,
    atype: u16,
    class: u16,
    ttl: u32,
    rdlength: u16,
    rdata: [u8; 4],
}

fn serialize_dns_message(message: DnsMessage) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(serialize_dns_header(&message.header));
    for question in message.question {
        buffer.extend(serialize_dns_question(&question));
    }
    for answer in message.answers {
        buffer.extend(serialize_dns_answer(&answer));
    }
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

fn serialize_dns_answer(answer: &DnsAnswer) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend(split_dns_name(&answer.name));
    buffer.extend(&answer.atype.to_be_bytes());
    buffer.extend(&answer.class.to_be_bytes());
    buffer.extend(&answer.ttl.to_be_bytes());
    buffer.extend(&answer.rdlength.to_be_bytes());
    buffer.extend(answer.rdata);
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

fn encode_ip_address(ip: &str) -> [u8; 4] {
    ip.parse::<std::net::Ipv4Addr>()
        .expect("Invalid IPv4 address")
        .octets()
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
                        ancount: 1,
                        nscount: 0,
                        arcount: 0,
                    },
                    question: vec![DnsQuestion {
                        name: b"codecrafters.io".to_vec(),
                        qtype: 1,
                        class: 1,
                    }],
                    answers: vec![DnsAnswer {
                        name: b"codecrafters.io".to_vec(),
                        atype: 1,
                        class: 1,
                        ttl: 3600,
                        rdlength: 4,
                        rdata: encode_ip_address("8.8.8.8"),
                    }],
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
