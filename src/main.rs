use std::net::{TcpListener, TcpStream};
use std::{println, io::{Read}};
// use std::time::Duration;
mod methods;

fn method_handler(mut stream: TcpStream) {
    let mut packet = [0; 65535];
    let bytes_read = stream.read(&mut packet[..]);
    match bytes_read {
        Ok(n) if n >= 7 => {
            if packet[0..3] == [0x47, 0x45, 0x54] {
                println!("GET Method");

                methods::get::handler(stream, &packet);

            } else if packet[0..4] == [0x48, 0x45, 0x41, 0x44] {
                println!("HEAD Method");
            } else if packet[0..6] == [0x4F, 0x50, 0x49, 0x4F, 0x4E, 0x53] {
                println!("OPTIONS Method");
            } else if packet[0..5] == [0x54, 0x52, 0x41, 0x43, 0x45] {
                println!("TRACE Method");
            } else if packet[0..3] == [0x50, 0x55, 0x54] {
                println!("PUT Method");
            } else if packet[0..6] == [0x44, 0x45, 0x4C, 0x45, 0x54, 0x45] {
                println!("DELETE Method");
            } else if packet[0..4] == [0x50, 0x4F, 0x53, 0x54] {
                println!("POST Method");
            } else if packet[0..5] == [0x50, 0x41, 0x54, 0x43, 0x48] {
                println!("PATCH Method");
            } else if packet[0..7] == [0x43, 0x4F, 0x4E, 0x4E, 0x45, 0x43, 0x54] {
                println!("CONNECT method");
            } else {
                println!("Unknown Method");
            }
        },
        Ok(_) => {
            println!("Packet appears too small");
        },
        Err(_e) => {
            println!("No data recieved");
        }
    }

}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                method_handler(stream);
            }
            Err(_e) => {
                println!("Couldn't conenct to client...");
            }
        }
    }
}
