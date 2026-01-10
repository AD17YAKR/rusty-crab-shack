use std::fs;

pub fn string_processing_meow(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let mut lines: usize = 0;
    let mut words: usize = 0;
    let bytes: usize = contents.len();

    let mut in_word = false;

    for ch in contents.chars() {
        print!("{}", ch)
    }
}
