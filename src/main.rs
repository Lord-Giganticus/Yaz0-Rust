use std::env;
use std::path::*;
use std::fs::*;
use std::io::*;
use std::str;

mod getfile;
mod yaz0;

fn main() {
    let envargs: Vec<String> = env::args().collect();
    let args = &envargs[1..];
    for arg in args {
        let mut buffer = getfile::get_file_as_byte_vec(arg);
        let magic = str::from_utf8(&buffer[0..4]).unwrap();
        if magic == "Yaz0" {
            buffer = yaz0::yaz0decomp(&buffer);
            let mut name = Path::new(arg).file_stem().unwrap().to_str().unwrap().to_owned();
            let ext = Path::new(arg).extension().unwrap().to_str().unwrap();
            name.push_str(".decomp.");
            name.push_str(ext);
            let mut file = File::create(name).unwrap();
            file.write(&buffer).expect("Write Failed:");
        } else {
            
        }
    }
}
