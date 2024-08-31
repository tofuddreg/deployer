use crate::generate_conf::file_struct;
use serde_json;
use std::{fs::File, io::Read};

pub fn run(path: &str) {
    let mut buf: Vec<u8> = Vec::new();
    let mut file = File::open(path).expect("Failed to open the config file.");
    let size: usize = file
        .read_to_end(&mut buf)
        .expect("Failed to read the config file.");

    println!("size: {:?}", size);
    let res: file_struct::ConfigFile =
        serde_json::from_slice(&buf).expect("Failed to parse json config.");

    dbg!(res);
}
