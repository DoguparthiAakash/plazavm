use std::path::Path;
fn main() {
    let path = Path::new("test.ext4");
    let mut fmt = arcbox_ext4::Formatter::new(&path, 4096, 100 * 1024 * 1024).unwrap();
    fmt.create(
        "/workspace",
        0o755 | 0x4000,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    fmt.close().unwrap();
    let meta = std::fs::metadata("test.ext4").unwrap();
    println!("File size: {}", meta.len());
}
