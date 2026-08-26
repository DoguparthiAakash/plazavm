#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use wasmtime::{Engine, Module};

    #[test]
    fn dump_imports() {
        let engine = Engine::default();
        let wasm_path = PathBuf::from("../../v86_investigation/v86.wasm");
        let module = Module::from_file(&engine, wasm_path).unwrap();

        for import in module.imports() {
            println!(
                "Import: module={}, name={}, type={:?}",
                import.module(),
                import.name(),
                import.ty()
            );
        }
    }
}
