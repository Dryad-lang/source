use std::fs;
use std::io::{self, Write};

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

pub fn append_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    write!(file, "{}", content)?;
    Ok(())
}