use backhand::{compression::Compressor, FilesystemCompressor, FilesystemWriter, NodeHeader};
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

fn main() {
    let mut writer = FilesystemWriter::default();
    writer.set_compressor(FilesystemCompressor::new(Compressor::Gzip, None).unwrap());

    // Test API
    let header = NodeHeader::default();
    writer
        .push_file(Cursor::new(b"hello"), "hello.txt", header.clone())
        .unwrap();
    writer.push_dir("etc", header.clone()).unwrap();
    writer
        .push_file(Cursor::new(b"world"), "etc/world.txt", header.clone())
        .unwrap();

    let mut out = File::create("test.sqsh").unwrap();
    writer.write(&mut out).unwrap();
    println!("OK");
}
