use std::env;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Write, Read, Error};


fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()) }
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let addr: String;

    if args.len() != 2 {
        addr = String::from("0.0.0.0:8888");
    } else {
        addr = format!("0.0.0.0:{}", args[1]);
    }
    
    // by default any client that can talk to this host they can talk on port 8888
    let listener = TcpListener::bind(addr).expect("Could not bind");

    // listener then continuously loops returning stream objects 
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("Failed: {}", e) },
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("Failed to handle: {:?}", error));
                });
            },
        }
    }

}
