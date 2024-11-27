use std::net::TcpStream;
use std::io::{Write};
use super::Methods;
use std::fs;

struct Packet {
    method: Methods,
    path: String,
    protocol_version: String,
}

pub fn handler(stream: TcpStream, binary_packet: &[u8]) {
    let string_packet = String::from_utf8_lossy(&binary_packet);
    let packet_breakdown: Vec<&str> = string_packet.lines().collect();
    let mut header = packet_breakdown[0].split_whitespace();

    let _ = header.next();


    let this_packet = Packet {
        method: Methods::GET,
        path: String::from(header.next().unwrap_or("/")),
        protocol_version: String::from(header.next().unwrap_or("/")),
    };

    println!("{}",string_packet);

    response(stream, this_packet);
}

fn response(mut stream: TcpStream, this_packet: Packet){
    let mut content = Vec::new();
    let code = get_content(this_packet.path, &mut content);
    match code{
        200 => { stream.write(&content); },
        404 => { stream.write("404 Page not found".as_bytes()); },
        _ => println!("Unknown Error Code"),
    };

}

fn get_content(path: String, content: &mut Vec<u8>) -> i32 {
    let mut this_path = ".".to_string() + &path;
    if this_path == "./" {
        this_path = "./index.html".to_string();
    }

    let data = match fs::read(this_path) {
        Ok(vec) => vec,
        Err(e) => return 404
    };

    content.extend_from_slice(data.as_slice());
    return 200;
}

// fn gen_header(){
//
// }
