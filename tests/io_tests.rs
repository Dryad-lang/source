// tests/io_tests.rs

use dryad::interpreter::io;

#[test]
fn test_write_and_read_file() {
    let path = "test_io.txt";
    io::write_file(path, "Dryad I/O Test").unwrap();
    let content = io::read_file(path).unwrap();
    assert_eq!(content, "Dryad I/O Test");
}

#[test]
fn test_append_file() {
    let path = "test_io_append.txt";
    io::write_file(path, "Line 1\n").unwrap();
    io::append_file(path, "Line 2\n").unwrap();
    let content = io::read_file(path).unwrap();
    assert_eq!(content, "Line 1\nLine 2\n");
}
