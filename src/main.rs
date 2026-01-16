use std::env;
use std::time::Instant;

mod basics;
mod head;
mod meow;
mod rusty_wc;
use basics::read_and_print_from_file::read_using_read_to_string;
use head::stream_processing::stream_processing_head;
use meow::bytes_processing::bytes_processing_meow;
use meow::stream_processing::stream_processing_meow;
use meow::string_processing::string_processing_meow;
use rusty_wc::bytes_processing::bytes_processing;
use rusty_wc::stream_processing::stream_processing;
use rusty_wc::string_processing::string_processing;

fn main() {
    basics();
}

fn head() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("Usage: mywc <filename>");
            std::process::exit(1);
        }
    };

    let n: usize = (match env::args().nth(2) {
        Some(f) => f,
        None => {
            eprintln!("Usage: mywc <filename>");
            std::process::exit(1);
        }
    })
    .parse()
    .unwrap();

    stream_processing_head(&filename, n);
}
fn basics() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("Usage: mywc <filename>");
            std::process::exit(1);
        }
    };

    read_using_read_to_string(&filename);
}
fn meow() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("Usage: mywc <filename>");
            std::process::exit(1);
        }
    };

    let start = Instant::now();
    stream_processing_meow(&filename);
    println!("\nTime taken: {} ms\n", start.elapsed().as_millis());
}

fn rusty_wc() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("Usage: mywc <filename>");
            std::process::exit(1);
        }
    };

    let start = Instant::now();
    bytes_processing(&filename);
    println!(
        "Time taken by bytes function: {} ms\n",
        start.elapsed().as_millis()
    );

    let start = Instant::now();
    string_processing(&filename);
    println!(
        "Time taken by String function: {} ms\n",
        start.elapsed().as_millis()
    );

    let start = Instant::now();
    stream_processing(&filename);
    println!(
        "Time taken by Stream function: {} ms\n",
        start.elapsed().as_millis()
    );
}
