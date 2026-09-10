#[allow(unused_imports)]
use std::net::UdpSocket;

struct DnsMessage {
    header: DnsHeader,
}

struct DnsHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
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
                        qdcount: 0,
                        ancount: 0,
                        nscount: 0,
                        arcount: 0,
                    },
                };

                let response = serialize_dns_header(&dns_msg.header);
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
