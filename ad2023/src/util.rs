use std::fs::File;
use std::io::prelude::*;

pub fn load_input(path: &str) -> String {
    let mut file = File::open(path).expect("File not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file to string");
    contents
}
