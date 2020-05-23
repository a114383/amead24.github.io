// Rust 2015 - enable macros in this crate
#[macro_use] extern crate serde_derive;
// Rust 2018 - serde_derive::{Serialize, Deserialize};
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::fmt::{Display, Formatter, Result};


// the `Debug` trait allows you to use the println!("{:?}", obj)
// which will generate the stdout code for you
#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    workers:        u64,
    ignore:         bool,
    auth_server:    Option<String>
}


// Or create your own display type
impl Display for ServerConfig {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "I shall not tell.")

    }
}


fn main() {
    let config = ServerConfig { workers: 100, ignore: false, auth_server: Some("auth.server.io".to_string()) };
    {
        println!("To and from YAML");
        let serialized = serde_yaml::to_string(&config).unwrap();
        println!("{}", serialized);
        let deserialized: ServerConfig = serde_yaml::from_str(&serialized).unwrap();
        //println!("{:?}", deserialized);
        println!("Printing Display Trait: {}", deserialized);
    }
    println!("\n\n");
    {
        println!("To and from JSON");
        let serialized = serde_json::to_string(&config).unwrap();
        println!("{}", serialized);
        let deserialized: ServerConfig = serde_json::from_str(&serialized).unwrap();
        println!("{:?}", deserialized);
    }
}