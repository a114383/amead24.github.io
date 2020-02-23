use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    // Note: When attempting to use '?'
    // https://stackoverflow.com/questions/48015600/cannot-use-operator-for-functions-that-return-result-error/48015721#48015721
    // "You can’t use try!/? because it causes the containing function to return the same Err,
    // but main() can’t return an Err (it returns (), not Result<…>).
    // pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpStream>
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        stream.write(input.as_bytes()).expect("Failed to write to server");

        // We letting BufReader borrow the stream to handle chunking,
        // and then letting it borrow buffer (Vector which grows as need)
        // to copy the contents of stdin into, which it then prints out
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();

        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}
