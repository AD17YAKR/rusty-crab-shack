use std::env;
use std::fs;

struct FileDetails {
    words: i32,
    lines: i32,
    character: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: mywc <filename>");
        return;
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let file_details = data_processing(&contents);
    println!(
        "The File {} has {} words, {} lines and {} characters",
        filename, file_details.words, file_details.lines, file_details.character
    );
}
fn data_processing(content: &str) -> FileDetails {
    let mut res = FileDetails {
        words: 0,
        lines: 0,
        character: 0,
    };

    let mut in_word = false;

    for it in content.chars() {
        res.character += 1;

        if it == '\n' {
            res.lines += 1;
        }

        if it.is_whitespace() {
            in_word = false;
        } else if !in_word {
            res.words += 1;
            in_word = true;
        }
    }

    if res.character > 0 {
        res.lines += 1;
    }

    res
}
