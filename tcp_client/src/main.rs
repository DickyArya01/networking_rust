use std::{io::{Read, Write}, net::TcpStream};

fn main() {
    //establish connection to server
    let connect = TcpStream::connect("127.0.0.1:8080");

    match connect {
        Ok(mut stream) => {

            let msg = String::from("Hello world");

            //write something to buffer for server to read 
            match stream.write(msg.as_bytes()){
                Ok(_) => {
                    println!("Write into stream: {}", msg);
                },
                Err(err) => {
                    eprintln!("Failed writing to stream: {}", err);
                    return;
                }
            }

            let mut buf = [0; 512];

            //read what ever server is spitting
            match stream.read(&mut buf) {
                Ok(n) => println!("Receive: {}", String::from_utf8_lossy(&buf[..n])),
                Err(err) => {
                    eprintln!("Error reading feedback from server: {}", err);
                    return;
                }
            }
        },
        Err(err) => {
            eprintln!("Error connecting to server: {}", err);
            return;
        }
    }
}
