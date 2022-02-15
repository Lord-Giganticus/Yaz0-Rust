extern crate indicatif;
extern crate thread_control;
pub mod yaz0;
pub mod getfile;
use std::fs::*;
use std::io::*;
use std::*;
use indicatif::*;
use thread_control::*;
pub fn decompress(data: &Vec<u8>) -> Vec<u8> {
    return yaz0::yaz0decomp(data);
}
pub fn compress(data: &Vec<u8>) -> Vec<u8> {
    return yaz0::yaz0comp(&data);
}
pub fn openanddecompress(filename: &String) -> Vec<u8> {
    let buffer = getfile::get_file_as_byte_vec(filename);
    return decompress(&buffer);
}
pub fn openandcompress(filename: &String) -> Vec<u8> {
    let buffer = getfile::get_file_as_byte_vec(filename);
    return compress(&buffer);
}
pub fn decompressandwrite(data: &Vec<u8>, path: &String) {
    let buffer = decompress(&data);
    let prog = ProgressBar::new(buffer.len() as u64);
    prog.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
    .progress_chars("#>-"));
    let mut file = File::create(path.as_str()).unwrap();
    let (flag, control) = make_pair();
    let handel = thread::spawn(move || {
        while flag.is_alive() {
            file.write(&buffer).unwrap();
            break;
        }
    });
    while !control.is_done() {
        prog.set_position(fs::metadata(path.as_str()).unwrap().len());
        if control.is_done() {
            control.stop();
            break;
        }
    }
    handel.join().unwrap();
    prog.finish_with_message("Downloaded!");
}
pub fn compressandwrite(data: &Vec<u8>, path: &String) {
    let buffer = compress(&data);
    let prog = ProgressBar::new(buffer.len() as u64);
    prog.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
    .progress_chars("#>-"));
    let mut file = File::create(path.as_str()).unwrap();
    let (flag, control) = make_pair();
    let handel = thread::spawn(move || {
        while flag.is_alive() {
            file.write(&buffer).unwrap();
            break;
        }
    });
    while !control.is_done() {
        prog.set_position(fs::metadata(path.as_str()).unwrap().len());
        if control.is_done() {
            control.stop();
            break;
        }
    }
    handel.join().unwrap();
    prog.finish_with_message("Downloaded!");
}
pub fn openandcompressandwrite(compressed: &String, path: &String) {
    let buffer = getfile::get_file_as_byte_vec(compressed);
    decompressandwrite(&buffer, path);
}
pub fn openanddecompressandwrite(compressed: &String, path: &String) {
    let buffer = getfile::get_file_as_byte_vec(compressed);
    compressandwrite(&buffer, path);
}