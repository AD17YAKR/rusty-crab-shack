# Rusty WC

Rust implementations of Unix commands `cat` and `wc` demonstrating different file processing techniques.

## What it does

### Cat (meow module)
Displays file contents using stream processing with `io::copy`

### WC (rusty_wc module) 
Counts lines, words, and characters using three approaches:
- **Bytes Processing**: Reads file into memory as bytes
- **String Processing**: Reads file as UTF-8 string  
- **Stream Processing**: Processes file in chunks using buffered reading

## Usage

```bash
cargo run <filename>
```

Example:
```bash
cargo run test.txt
```

## Learning Goals

- File I/O in Rust
- Different text processing approaches
- Performance comparison between methods
- Error handling with `unwrap()` and `expect()`
- Modular code organization

## Sample Output

```
[File contents displayed]

Time taken: 1 ms
```