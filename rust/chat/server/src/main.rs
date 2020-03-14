// use std::collections::HashMap;
// use std::env;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

// TODO: Make into nicer CLI

//let args: Vec<String> = env::args().collect();
//let pargs: HashMap<&str, &str> = parse_args(args);

//let URL: &str = format!("127.0.0.1:{}", pargs.entry("--port").or_default("8888"));
//let MSIZE: usize = pargs.entry("--buff").or_default(128).parse().unwrap();

//fn parse_args(args: Vec<String>) -> HashMap<&'str, String> {
//    let mut pargs: HashMap<&str, String> = HashMap::new();
//
//    for arg in args {
//        let temp: Vec<&str> = arg.split("=").collect();
//
//        if temp.len() == 2 {
//            pargs.insert(temp[0], temp[1].to_string());
//        } else {
//            println!("idk what you're doing with {:?}", arg);
//        }
//    }
//
//    pargs
//}
//


const URL: &str = "127.0.0.1:8888";
const MSIZE: usize = 32;


fn sleep() {
    thread::sleep(std::time::Duration::from_millis(100));
}


fn main() {
    let listener = TcpListener::bind(URL).expect("Could not bind");
    listener.set_nonblocking(true).expect("Count not enable non-blocking");

    // Further reading: https://doc.rust-lang.org/std/sync/index.html
    let mut clients: Vec<TcpStream> = Vec::new();

    loop {
        match listener.accept() {
            Err(e) => println!("Client connection failed: {:?}", e),
            Ok((mut socket, addr)) => {
                println!("Client {} connected.", addr);
               
                clients.push(socket);
                
                let mut buffer = vec![0; 512];
                let bytes_read = socket.read(&mut buffer);

                for client in clients { client.write(&buffer); }
            },
        }
    }
}
