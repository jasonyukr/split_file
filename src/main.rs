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
    if let Ok(file) = File::create(a_filename) {
        a_file = file;
    } else {
        return;
    }

    let b_file;
    if let Ok(file) = File::create(b_filename) {
        b_file = file;
    } else {
        return;
    }

    let mut a_writer = BufWriter::new(&a_file);
    let mut b_writer = BufWriter::new(&b_file);

    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        if let Ok(line) = ln {
            if let Some(i) = line.find(&separator) {
                let idx = i as usize;
                let _ = a_writer.write(&line[..idx].as_bytes());
                let _ = a_writer.write("\n".as_bytes());
                let _ = b_writer.write(&line[idx+separator.len()..].as_bytes());
                let _ = b_writer.write("\n".as_bytes());
            } else {
                let _ = b_writer.write(line.as_bytes());
                let _ = b_writer.write("\n".as_bytes());
            }
        }
    }
}
