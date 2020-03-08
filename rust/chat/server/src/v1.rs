use std::collections::HashMap;
use std::env;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;


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
    thread::sleep_ms(100);
}


fn main() {
    //let args: Vec<String> = env::args().collect();
    //let pargs: HashMap<&str, &str> = parse_args(args);

    //let URL: &str = format!("127.0.0.1:{}", pargs.entry("--port").or_default("8888"));
    //let MSIZE: usize = pargs.entry("--buff").or_default(128).parse().unwrap();

    let listener = TcpListener::bind(URL).expect("Could not bind");
    listener.set_nonblocking(true).expect("Count not enable non-blocking");

    // Further reading: https://doc.rust-lang.org/std/sync/index.html
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = listener.accept() { // fn accept -> Result<TcpStream, SocketAddr)
            println!("Client {} connected", addr);

            // We clone it so that we can move it into a thread
            let tx = tx.clone();
            thread::spawn(move || loop {
                // fill a vector of length MSIZE with zeros
                let mut buff = vec![0; MSIZE];

                // borrow the buffer and mutate it
                match socket.read_exact(&mut buff) { // -> Result()
                    Ok(_) => {
                        // loop through the buffer, while not zero (default)
                        // and turn it into a Vector? - Which it already is?
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing: {}", addr);
                        break;
                    }
                }

                sleep();

            });
        }

        if let Ok(msg) = rx.try_recv() {
            // try and recieve the messsage from the channel and if you get one,
            // pass it into this closure func which is the logic of the filter

            clients = clients.into_iter().filter_map(|mut client: TcpStream| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSIZE, 0);

                // What's the type of client?!
                client.write_all(&buff).map(|_| client).ok()

            }).collect::<Vec<_>>();
        }

        sleep();
    }
}
