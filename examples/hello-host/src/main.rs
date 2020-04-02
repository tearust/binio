use wasmtime::{Store, Module, Extern, Instance};
use wasmtime_wasi::{Wasi, WasiCtxBuilder};
use wasi_binio_host::{call_stub};
use hello_shared::{Point, Rect};//The stuct put into shared mod because we want the wasm and host use exactly the same struct

fn main() {
    let store = Store::default();
    let module = Module::from_file(&store, "../hello-wasm/target/wasm32-wasi/release/hello_wasm.wasm").expect("load wasm file error");

    //These lines are standard Wasi init
    let wcb = {
        WasiCtxBuilder::new()
        .inherit_stdio()//support basic io for wasm module so that the println! will work inside wasm module
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
                println!("This example only support wasi_snapshot_preview1 at this moment");
            },
        }
    }
    //this is standard Wasi init
    let instance = Instance::new(&module, &imports).unwrap();
    
    //We create two Point values here
    let point1 = Point{x:2, y:3};
    let point2 = Point{x:8, y:9};
    
    println!("Host app created two Point values which will be sent to the wasm module: {:?} {:?}", point1, point2);

    //we use binio function call_stub to indirectly call the 'do_compute' function in 
    //our wasm module.
    //Because wasm function cannot take (point1, point2) as input args so 
    //we have to indirectly using call_stub
    //call_stub takes the do_compute's args as its args, then returns whatever
    //do_compute will return
    let result: Rect/* It is very important to have the Type here */
        = call_stub(&instance, //Instance is needed to find export wasm function
            &(point1, point2),//This was originally 'do_compute' args, now used by call_stub, it will then be transferred to wasm
            "do_compute"//this is the name of the user function in wasm.
        );
            
    println!("Host app received wasm module return {:?}", result);
}
