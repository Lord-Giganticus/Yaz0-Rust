extern crate indicatif;
extern crate thread_control;
use std::path::*;
use std::fs::*;
use std::io::*;
use std::str;
use std::*;
use indicatif::*;
use thread_control::*;

mod getfile;
mod yaz0;

fn main() {
    let envargs: Vec<String> = env::args().collect();
    let args = &envargs[1..];
    for arg in args {
        let mut buffer = getfile::get_file_as_byte_vec(arg);
        let magic = str::from_utf8(&buffer[0..4]).unwrap();
        let mut name = Path::new(arg).file_stem().unwrap().to_str().unwrap().to_owned();
        let ext = Path::new(arg).extension().unwrap().to_str().unwrap();
        let (flag, control) = make_pair();
        if magic == "Yaz0" {
            buffer = yaz0::yaz0decomp(&buffer);
            let prog = ProgressBar::new(buffer.len() as u64);
            prog.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
            .progress_chars("#>-"));
            name.push_str(".decomp.");
            name.push_str(ext);
            let mut truename = Path::new(arg).file_stem().unwrap().to_str().unwrap().to_owned();
            truename.push_str(".decomp.");
            truename.push_str(ext);
            let mut file = File::create(name).unwrap();
            let handel = thread::spawn(move || {
                while flag.is_alive() {
                    file.write(&buffer).unwrap();
                    break;
                }
            });
            while !control.is_done() {
                prog.set_position(fs::metadata(truename.as_str()).unwrap().len());
                if control.is_done() {
                    control.stop();
                    break;
                }
            }
            handel.join().unwrap();
            prog.finish_with_message("Downloaded!");
        } else {
            buffer = yaz0::yaz0comp(&buffer);
            let prog = ProgressBar::new(buffer.len() as u64);
            prog.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
            .progress_chars("#>-"));
            name.push_str(".comp.");
            name.push_str(ext);
            let mut truename = Path::new(arg).file_stem().unwrap().to_str().unwrap().to_owned();
            truename.push_str(".comp.");
            truename.push_str(ext);
            let mut file = File::create(name).unwrap();
            let handel = thread::spawn(move || {
                while flag.is_alive() {
                    file.write(&buffer).unwrap();
                    break;
                }
            });
            while !control.is_done() {
                prog.set_position(fs::metadata(truename.as_str()).unwrap().len());
                if control.is_done() {
                    control.stop();
                    break;
                }
            }
            handel.join().unwrap();
            prog.finish_with_message("Downloaded!");
        }
    }
}
