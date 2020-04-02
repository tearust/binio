use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtxBuilder};
use binio_host::{call_stub};
//use serde::{Serialize, Deserialize};
use hello_shared::{Point, React};

fn main() {
    let store = Store::default();
    let module = Module::from_file(&store, "../hello-wasm/target/wasm32-wasi/release/hello_wasm.wasm").expect("load wasm file error");

    let point1 = Point{x:2, y:3};
    let point2 = Point{x:8, y:9};
    let wcb = {
        WasiCtxBuilder::new()
        .env("HOME", "DIR")
        .inherit_stdio()
        .build().expect("error here")
    };
    let wasi = Wasi::new(&store, wcb);
    let mut imports = Vec::new();
    for import in module.imports() {
        match import.module(){
            "wasi_snapshot_preview1" => {
                if let Some(export) = wasi.get_export(import.name()) {
                    imports.push(Extern::from(export.clone()));
                }
                else{
                    println!(
                        "couldn't find import for `{}::{}`",
                        import.module(),
                        import.name()
                    );
                }
            },
            _default => {
                
                println!(
                    "couldn't find import for `{}::{}`",
                    import.module(),
                    import.name()
                );
            },
        }
    }

    let instance = Instance::new(&module, &imports).unwrap();
    let result: React= call_stub(&instance, &(point1, point2), "do_compute");
    println!("return React {:?}", result);
}
