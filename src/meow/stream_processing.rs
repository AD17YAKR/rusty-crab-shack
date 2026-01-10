use std::fs::File;
use std::io::{self};

pub fn stream_processing_meow(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut stdout = io::stdout().lock();

    io::copy(&mut file, &mut stdout).unwrap();
}
