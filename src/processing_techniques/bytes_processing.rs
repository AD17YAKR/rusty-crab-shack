use std::fs::File;
use std::io::Read;

pub fn bytes_processing(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut lines = 0;
    let mut words = 0;
    let mut in_word = false;

    for &byte in &buffer {
        if byte == b'\n' {
            lines += 1;
        }

        let is_whitespace = byte == b' ' || byte == b'\n' || byte == b'\t';

        if !is_whitespace && !in_word {
            words += 1;
            in_word = true;
        }

        if is_whitespace {
            in_word = false;
        }
    }

    let bytes = buffer.len();
    println!("Lines\tWords\tCharacters\tFileName");
    println!("{:>5}\t{:>5}\t{:>10}\t{}", lines, words, bytes, filename);
}
