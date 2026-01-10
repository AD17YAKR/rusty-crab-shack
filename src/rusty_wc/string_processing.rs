use std::fs;

pub fn string_processing(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let mut lines: usize = 0;
    let mut words: usize = 0;
    let bytes: usize = contents.len();

    let mut in_word = false;

    for ch in contents.chars() {
        if ch == '\n' {
            lines += 1;
        }

        if ch.is_whitespace() {
            in_word = false;
        } else if !in_word {
            words += 1;
            in_word = true;
        }
    }

    println!("Lines\tWords\tCharacters\tFileName");
    println!("{:>5}\t{:>5}\t{:>10}\t{}", lines, words, bytes, filename);
}
