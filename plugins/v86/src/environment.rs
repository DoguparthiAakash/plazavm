use plaza_foundation::core::PlazaResult;
use plaza_runtime::storage::RuntimeStorage;
use wasmtime::{Caller, Engine, Extern, Func, Linker, Memory, MemoryType, Module, Ref, RefType, HeapType, Store, Table, TableType};

pub struct V86State {
    pub storage: Option<RuntimeStorage>,
}

pub struct V86Environment {
    engine: Engine,
    linker: Linker<V86State>,
    module: Module,
}

impl V86Environment {
    pub fn new(wasm_path: &std::path::Path) -> PlazaResult<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, wasm_path).map_err(|e| {
            plaza_foundation::core::PlazaError::process(format!("Failed to load v86 WASM: {}", e))
        })?;

        let mut linker = Linker::new(&engine);

        // ── Function imports ─────────────────────────────────────────

        // 1. get_rand_int () -> I32
        linker.func_wrap("env", "get_rand_int", |_: Caller<'_, V86State>| -> i32 {
            0 // Simple deterministic random for now
        }).unwrap();

        // 2. mmap_read8 (I32) -> I32
        linker.func_wrap("env", "mmap_read8", |_: Caller<'_, V86State>, _: i32| -> i32 {
            0
        }).unwrap();

        linker.func_wrap("env", "mmap_read32", |_: Caller<'_, V86State>, _: i32| -> i32 {
            0
        }).unwrap();

        linker.func_wrap("env", "mmap_write16", |_: Caller<'_, V86State>, _: i32, _: i32| {}).unwrap();

        // 3. mmap_write8 (I32, I32)
        linker.func_wrap("env", "mmap_write8", |_: Caller<'_, V86State>, _: i32, _: i32| {}).unwrap();

        // 4. io_port_write16 (I32, I32)
        linker.func_wrap("env", "io_port_write16", |_: Caller<'_, V86State>, _: i32, _: i32| {}).unwrap();

        // 5. stop_idling ()
        linker.func_wrap("env", "stop_idling", |_: Caller<'_, V86State>| {}).unwrap();

        // 6. io_port_write32 (I32, I32)
        linker.func_wrap("env", "io_port_write32", |_: Caller<'_, V86State>, _: i32, _: i32| {}).unwrap();

        // 7. io_port_read16 (I32) -> I32
        linker.func_wrap("env", "io_port_read16", |_: Caller<'_, V86State>, _: i32| -> i32 { 0 }).unwrap();

        // 8. io_port_read32 (I32) -> I32
        linker.func_wrap("env", "io_port_read32", |_: Caller<'_, V86State>, _: i32| -> i32 { 0 }).unwrap();

        // 9. cpu_event_halt ()
        linker.func_wrap("env", "cpu_event_halt", |_: Caller<'_, V86State>| {}).unwrap();

        // 10. run_hardware_timers (I32, F64) -> F64
        linker.func_wrap("env", "run_hardware_timers", |_: Caller<'_, V86State>, _: i32, _: f64| -> f64 { 0.0 }).unwrap();

        // 11. mmap_write32 (I32, I32)
        linker.func_wrap("env", "mmap_write32", |_: Caller<'_, V86State>, _: i32, _: i32| {}).unwrap();

        // 12. mmap_write64 (I32, I32, I32)
        linker.func_wrap("env", "mmap_write64", |_: Caller<'_, V86State>, _: i32, _: i32, _: i32| {}).unwrap();

        // 13. mmap_write128 (I32, I32, I32, I32, I32)
        linker.func_wrap("env", "mmap_write128", |_: Caller<'_, V86State>, _: i32, _: i32, _: i32, _: i32, _: i32| {}).unwrap();

        // 14. io_port_read8 (I32) -> I32
        linker.func_wrap("env", "io_port_read8", |_: Caller<'_, V86State>, _: i32| -> i32 { 0 }).unwrap();

        // 15. io_port_write8 (I32, I32)
        linker.func_wrap("env", "io_port_write8", |_: Caller<'_, V86State>, _: i32, _: i32| {}).unwrap();

        // 16. jit_clear_func (I32)
        linker.func_wrap("env", "jit_clear_func", |_: Caller<'_, V86State>, _: i32| {}).unwrap();

        // 17. codegen_finalize (I32, I32, I32, I32, I32)
        linker.func_wrap("env", "codegen_finalize", |_: Caller<'_, V86State>, _: i32, _: i32, _: i32, _: i32, _: i32| {}).unwrap();

        // 18. console_log_from_wasm (I32, I32)
        linker.func_wrap("env", "console_log_from_wasm", |mut caller: Caller<'_, V86State>, ptr: i32, len: i32| {
            if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
                let data = mem.data(&caller);
                let start = ptr as usize;
                let end = start + len as usize;
                if start < data.len() && end <= data.len() {
                    if let Ok(s) = std::str::from_utf8(&data[start..end]) {
                        println!("v86: {}", s);
                    }
                }
            }
        }).unwrap();

        // 19. microtick () -> F64
        linker.func_wrap("env", "microtick", |_: Caller<'_, V86State>| -> f64 {
            // Simplified tick counter (could use actual timestamp later)
            0.0
        }).unwrap();

        // 20. log_wasm_debug (I32, I32)
        linker.func_wrap("env", "log_wasm_debug", |_: Caller<'_, V86State>, _: i32, _: i32| {}).unwrap();

        // ── Note: memory and __indirect_function_table are defined at
        //    instantiation time because they require a Store. ─────────

        Ok(Self {
            engine,
            linker,
            module,
        })
    }
    
    /// Return a reference to the engine (needed to create a Store externally).
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn instantiate(&self, store: &mut Store<V86State>) -> PlazaResult<wasmtime::Instance> {
        let mut linker = self.linker.clone();

        // ── Extract expected types from module imports ───────────────────────
        let mut mem_ty = MemoryType::new(16, Some(2048));
        let mut table_ty = TableType::new(RefType::new(true, HeapType::Func), 0, None);

        for import in self.module.imports() {
            if import.module() == "env" {
                if import.name() == "memory" {
                    if let wasmtime::ExternType::Memory(mt) = import.ty() {
                        mem_ty = mt;
                    }
                }
                if import.name() == "__indirect_function_table" {
                    if let wasmtime::ExternType::Table(tt) = import.ty() {
                        table_ty = tt;
                    }
                }
            }
        }

        // ── Memory import ────────────────────────────────────────────
        let memory = Memory::new(&mut *store, mem_ty).map_err(|e| {
            plaza_foundation::core::PlazaError::process(format!("Failed to create memory: {}", e))
        })?;
        linker.define(&*store, "env", "memory", memory).map_err(|e| {
            plaza_foundation::core::PlazaError::process(format!("Failed to define memory: {}", e))
        })?;

        // ── Indirect function table import ───────────────────────────
        let init_val = Ref::Func(None);
        let table = Table::new(&mut *store, table_ty, init_val).map_err(|e| {
            plaza_foundation::core::PlazaError::process(format!("Failed to create table: {}", e))
        })?;
        linker.define(&*store, "env", "__indirect_function_table", table).map_err(|e| {
            plaza_foundation::core::PlazaError::process(format!("Failed to define table: {}", e))
        })?;

        linker.instantiate(&mut *store, &self.module).map_err(|e| {
            plaza_foundation::core::PlazaError::process(format!("Failed to instantiate v86: {}", e))
        })
    }
}
