use std::fs::File;
use std::io::Read;
use wasmparser::{Parser, Payload};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://unpkg.com/v86@latest/build/v86.wasm";
    let file_path = "v86.wasm";

    if !std::path::Path::new(file_path).exists() {
        println!("Downloading {}...", url);
        let mut response = reqwest::blocking::get(url)?;
        let mut file = File::create(file_path)?;
        response.copy_to(&mut file)?;
        println!("Download complete.");
    }

    let mut file = File::open(file_path)?;
    let mut wasm_bytes = Vec::new();
    file.read_to_end(&mut wasm_bytes)?;

    println!("\nv86 WASM Compatibility Report");
    println!("=============================");

    let mut num_imports = 0;
    let mut num_exports = 0;

    let mut browser_dependent = vec![];

    for payload in Parser::new(0).parse_all(&wasm_bytes) {
        match payload? {
            Payload::ImportSection(s) => {
                for import in s {
                    let import = import?;
                    num_imports += 1;

                    let name = format!("{}.{}", import.module, import.field.unwrap_or(""));

                    browser_dependent.push(name.clone());

                    println!("Import: {}", name);
                }
            }
            Payload::ExportSection(s) => {
                for export in s {
                    let export = export?;
                    num_exports += 1;
                    println!("Export: {}", export.field);
                }
            }
            _ => {}
        }
    }

    println!("\nSummary:");
    println!("Total Imports: {}", num_imports);
    println!("Total Exports: {}", num_exports);

    println!("\nAll Imports:");
    for js_dep in browser_dependent.iter() {
        println!("  - {}", js_dep);
    }

    Ok(())
}
