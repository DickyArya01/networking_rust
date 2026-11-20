use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    //listening to port
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => {
            println!("Listen to port 8080");
            listener
        }
        Err(err) => {
            eprintln!("Error listening to port: {}", err);
            return;
        }
    };

    //iterator for incoming client
    for result in listener.incoming() {
        match result {
            Ok(stream) => handle_connection(stream),
            Err(err) => {
                eprintln!("Error acception incoming client: {}", err);
            }
        }
    }
}

//handle client connection
fn handle_connection(mut stream: TcpStream) {
    //init buf for stream.read to save on. initial value is 0 and it length is 512 (which is 0.5kb)
    let mut buf = [0; 512];

    loop {
        // read the stream
        match stream.read(&mut buf) {
            Ok(0) => break, // result 0 means client connestion is closed
            Ok(n) => {
                //write the buffer sent by client 
                stream.write(&buf[..n]).unwrap();
                println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
            }
            Err(err) => {
                eprintln!("Error reading client stream: {}", err);
                break;
            }
        }
    }
}
