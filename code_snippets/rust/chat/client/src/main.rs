use std::io::{self, ErrorKind, Read, Write};
use std::net::{TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:8888";
const MSIZE: usize = 32;


fn main(){

    let mut client = TcpStream::connect(LOCAL).expect("Failed to connect.");
    client.set_nonblocking(true).expect("failed to set non-binding");

    // Channel will only be transporting strings back and forth
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection dropped.");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSIZE, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("message sent {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep_ms(100);
    });

    println!("Write a message: ");

    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");

        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() { break }
    }

    println!("Bye bye!");

}