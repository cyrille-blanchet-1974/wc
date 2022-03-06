use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_read_file(to_compute: Sender<String>, fic: &str) -> JoinHandle<()> {
    let file = String::from(fic);
    spawn(move || {
        let input = File::open(&file);
        match input {
            Err(e) => {
                println!("Error reading file {} => {}", &file, e);
            }
            Ok(f) => {
                let buffered = BufReader::new(f);
                let mut num_line = 0;
                for line in buffered.lines() {
                    num_line += 1;
                    match line {
                        Ok(l) => {
                            if to_compute.send(l).is_err() {
                                println!("error sending to search");
                                return;
                            }
                        }
                        Err(e) => {
                            println!("Error reading file {} line {}=> {}", &file, num_line, e);
                        }
                    }
                }
            }
        }
    })
}

pub fn start_thread_read_stdin(to_compute: Sender<String>) -> JoinHandle<()> {
    let stdin = io::stdin(); // We get `Stdin` here.
    spawn(move || {
        let buffered = BufReader::new(stdin);
        let mut num_line = 0;
        for line in buffered.lines() {
            num_line += 1;
            match line {
                Ok(l) => {
                    if to_compute.send(l).is_err() {
                        println!("error sending to search");
                        return;
                    }
                }
                Err(e) => {
                    println!("Error reading stdin line {}=> {}", num_line, e);
                }
            }
        }
    })
}
