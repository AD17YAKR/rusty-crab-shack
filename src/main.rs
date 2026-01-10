use std::env;
use std::time::Instant;

mod meow;
mod rusty_wc;
use meow::bytes_processing::bytes_processing_meow;
use meow::stream_processing::stream_processing_meow;
use meow::string_processing::string_processing_meow;
use rusty_wc::bytes_processing::bytes_processing;
use rusty_wc::stream_processing::stream_processing;
use rusty_wc::string_processing::string_processing;

fn main() {
    meow();
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
