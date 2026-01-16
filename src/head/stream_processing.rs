use std::fs::File;
use std::io::{self, BufReader, Read, Write};

// pub fn stream_processing_head(filename: &str, max_lines: usize) -> io::Result<()> {
//     let file = File::open(filename)?;
//     let mut reader = BufReader::new(file);

//     Ok(())
// }
pub fn stream_processing_head(filename: &str, max_lines: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut stdout = io::stdout().lock();

    let mut buffer = [0u8; 8192];
    let mut lines = 0;
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        let mut end = n;
        for i in 0..n {
            if buffer[i] == b'\n' {
                lines += 1;
                if lines == max_lines {
                    end = i + 1;
                    stdout.write_all(&buffer[..end])?;
                    return Ok(());
                }
            }
        }

        stdout.write_all(&buffer[..end])?;
    }
    Ok(())
}
