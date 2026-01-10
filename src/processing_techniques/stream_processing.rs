use std::fs::File;
use std::io::{BufReader, Read};

pub fn stream_processing(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut buffer = [0u8; 8192];

    let mut lines: usize = 0;
    let mut words: usize = 0;
    let mut bytes: usize = 0;
    let mut in_word = false;

    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        bytes += n;

        for &b in &buffer[..n] {
            if b == b'\n' {
                lines += 1;
            }

            let is_whitespace = b == b' ' || b == b'\n' || b == b'\t';

            if !is_whitespace && !in_word {
                words += 1;
                in_word = true;
            }

            if is_whitespace {
                in_word = false;
            }
        }
    }

    println!("Lines\tWords\tCharacters\tFileName");
    println!("{:>8} {:>8} {:>8} {}", lines, words, bytes, filename);
}
