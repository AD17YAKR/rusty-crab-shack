use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("Usage: mywc <filename>");
            std::process::exit(1);
        }
    };

    let start = Instant::now();
    read_txt_file(&filename);
    println!(
        "Time taken by bytes function: {} ms\n",
        start.elapsed().as_millis()
    );

    let start = Instant::now();
    process_data_using_string_functions(&filename);
    println!(
        "Time taken by String function: {} ms\n",
        start.elapsed().as_millis()
    );
}

fn read_txt_file(filename: &str) {
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

struct FileDetails {
    words: usize,
    lines: usize,
    bytes: usize,
}

fn process_data_using_string_functions(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let mut details = FileDetails {
        words: 0,
        lines: 0,
        bytes: contents.len(), // bytes, not chars
    };

    let mut in_word = false;

    for ch in contents.chars() {
        if ch == '\n' {
            details.lines += 1;
        }

        if ch.is_whitespace() {
            in_word = false;
        } else if !in_word {
            details.words += 1;
            in_word = true;
        }
    }

    println!("Lines\tWords\tCharacters\tFileName");
    println!(
        "{:>5}\t{:>5}\t{:>10}\t{}",
        details.lines, details.words, details.bytes, filename
    );
}


