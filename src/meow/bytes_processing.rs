use std::fs::File;
use std::io::Read;

// pub fn bytes_processing_rustywc(filename: &str) {
//     let mut buffer = [0u8; 8192];
//     let mut stdout = io::stdout().lock();

//     loop {
//         let n = file.read(&mut buffer)?;
//         if n == 0 {
//             break;
//         }
//         stdout.write_all(&buffer[..n])?;
//     }
// }

use std::io::Write;

pub fn bytes_processing_meow(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    std::io::stdout().write_all(&buffer).unwrap();
}
