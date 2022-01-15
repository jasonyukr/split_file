use std::io::{self, BufRead, BufWriter};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // parse argument
    let mut sep_mode = false;
    let mut separator: String = String::from(":");
    let mut exec_name: String = String::from("");
    let mut a_filename: String = String::from("");
    let mut b_filename: String = String::from("");
    for arg in env::args() {
        if sep_mode {
            separator = arg.clone();
            sep_mode = false;
            continue;
        }
        if arg == "-d" || arg == "--d" {
            sep_mode = true;
        } else if exec_name == "" {
            exec_name = arg.clone();
        } else if a_filename == "" {
            a_filename = arg.clone();
        } else if b_filename == "" {
            b_filename = arg.clone();
        }
    }

    if a_filename == "" || b_filename == "" {
        return;
    }

    let a_file;
    match File::create(a_filename) {
        Ok(file) => a_file = file,
        Err(_) => return
    }

    let b_file;
    match File::create(b_filename) {
        Ok(file) => b_file = file,
        Err(_) => return
    }

    let mut a_writer = BufWriter::new(&a_file);
    let mut b_writer = BufWriter::new(&b_file);

    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        let line;
        match ln {
            Ok(data) => line = data,
            Err(_) => continue
        }
        let r = line.find(&separator);
        if let Some(idx) = &r {
            let _idx = *idx as usize;
            a_writer.write(&line[.._idx].as_bytes()).unwrap();
            a_writer.write("\n".as_bytes()).unwrap();
            b_writer.write(&line[_idx+separator.len()..].as_bytes()).unwrap();
            b_writer.write("\n".as_bytes()).unwrap();
        } else {
            b_writer.write(line.as_bytes()).unwrap();
            b_writer.write("\n".as_bytes()).unwrap();
        }
    }
}
