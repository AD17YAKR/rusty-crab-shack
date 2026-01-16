use std::fs;

pub fn read_using_read_to_string(filename: &str) {
    let mut contents =
        fs::read_to_string(filename).expect("Should have been able to read the file");
    contents = contents.chars().filter(|&c| c != 'f').collect();
    println!("{}", contents);
}
