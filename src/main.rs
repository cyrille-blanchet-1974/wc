mod paramcli;
mod read;
mod wc;

use paramcli::*;
use read::*;
use std::sync::mpsc::channel;
use wc::*;

pub fn traitement(p: &Paramcli) {
    //MPSC chanels
    let (to_compute, from_read) = channel();

    let hread = if !p.fic.is_empty() {
        start_thread_read_file(to_compute, &p.fic)
    } else {
        start_thread_read_stdin(to_compute)
    };
    let hcompute = start_thread_wc(from_read, &p.count_line);

    //wait for threads to stop
    if hread.join().is_err() {
        println!("Thread read finished with error");
    }
    if hcompute.join().is_err() {
        println!("Thread compute finished with error");
    }
}

fn main() {
    let param = Paramcli::new();
    traitement(&param);
}
