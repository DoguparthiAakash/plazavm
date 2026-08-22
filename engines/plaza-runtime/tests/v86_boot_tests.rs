//! Progressive Boot Acceptance Tests for the v86 Wasmtime backend.
//!
//! Test 1: WASM artifact loads into Wasmtime.
//! Test 2: All env imports resolve.
//! Test 3: WASM module instantiates with V86Environment.

use std::path::PathBuf;
use wasmtime::{Engine, Module, Store};
use plaza_runtime::backends::v86::environment::{V86Environment, V86State};

fn v86_wasm_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../v86_investigation/v86.wasm")
}

// ─── Test 1: WASM artifact loads ─────────────────────────────────────

#[test]
fn test_1_wasm_artifact_loads() {
    let path = v86_wasm_path();
    if !path.exists() {
        println!("v86.wasm not found at {:?}, skipping", path);
        return;
    }

    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).unwrap();
    println!("v86.wasm loaded: {} imports", module.imports().len());
    for import in module.imports() {
        println!("Import: module={}, name={}, type={:?}", import.module(), import.name(), import.ty());
    }
}

// ─── Test 2: All imports resolve ─────────────────────────────────────

#[test]
fn test_2_all_imports_resolve() {
    let path = v86_wasm_path();
    if !path.exists() {
        println!("v86.wasm not found at {:?}, skipping", path);
        return;
    }

    let env = V86Environment::new(&path);
    assert!(env.is_ok(), "V86Environment::new failed: {:?}", env.err());
    println!("All v86 env imports resolved successfully.");
}

// ─── Test 3: WASM instantiates ───────────────────────────────────────

#[tokio::test]
async fn test_3_wasm_instantiates() {
    let path = v86_wasm_path();
    if !path.exists() {
        println!("v86.wasm not found at {:?}, skipping", path);
        return;
    }

    let env = V86Environment::new(&path).unwrap();
    let mut store = Store::new(env.engine(), V86State { storage_bridge: None });
    let instance = env.instantiate(&mut store).await;
    assert!(instance.is_ok(), "v86 instantiation failed: {:?}", instance.err());

    // Verify we can find exported functions in the instance
    let instance = instance.unwrap();
    let exports: Vec<String> = instance
        .exports(&mut store)
        .map(|e| e.name().to_string())
        .collect();
    println!("v86 exports ({}):", exports.len());
    for name in &exports {
        println!("  - {}", name);
    }
    assert!(!exports.is_empty(), "v86 instance should have exports");
}

#[tokio::test]
async fn test_4_v86_initializes() {
    let path = v86_wasm_path();
    if !path.exists() {
        return;
    }

    let env = V86Environment::new(&path).unwrap();
    let mut store = Store::new(env.engine(), V86State { storage_bridge: None });
    let instance = env.instantiate(&mut store).await.unwrap();

    let rust_init = instance.get_func(&mut store, "rust_init");
    if let Some(func) = rust_init {
        println!("Found rust_init");
        if func.ty(&store).params().len() == 0 {
            func.call(&mut store, &[], &mut []).expect("rust_init failed");
            println!("rust_init called successfully");
        } else {
            println!("rust_init requires params");
        }
    } else {
        println!("rust_init not found");
    }
}

