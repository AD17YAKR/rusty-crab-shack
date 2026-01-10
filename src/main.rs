use std::env;
use std::time::Instant;

mod processing_techniques;
use processing_techniques::bytes_processing::bytes_processing;
use processing_techniques::stream_processing::stream_processing;
use processing_techniques::string_processing::string_processing;

fn main() {
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
