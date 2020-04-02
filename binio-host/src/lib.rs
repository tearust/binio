//! # wasi_binio_host and wasi_binio_wasm
//!
//! `wasi_binio_host` is the host crate of wasm_binio. Another crate is `wasi_binio_wasm` which used in wasm module.
//! It creates a call_stub so that host can call a webassembly function with complex data structure arguments and results.
//!
//! As of today, wasm function can only take i32 i64 f32 f64 types. If you have a function in wasm like this
//! In wasm: ` do_compute(point1: &Point, point2: &Point)->Rect {...}`
//! there is no way to call it directly from host.
//! With the help from wasm_binio, we can instead call the call_stub function and send arguments and results like this
//! 
//! Code in host: 
//! ```
//! let result: Rect = call_stub(&instance, &(point1, point2), "do_compute")
//! ```
//! 
//! Code in wasm: 
//!  ```
//! #[no_mangle]
//! fn do_compute(ptr:i32, buffer_size: i32)->i64{
//!     let point_tuple : (Point, Point) = wasi_binio_wasm::wasm_deserialize(ptr, buffer_size).unwrap();
//!     let rect: Rect = some_how_make_a_rect_from_two_points()... /* Your own logic here */
//!     wasi_binio_wasm::wasm_serialize(&rect).unwrap()
//! }
//! Since wasm and wasi are under active development. Wasi will soon provide complex arguments support.
//! At the time you find this crate, this feature probably have already been supported natively.
//! 
    

use wasmtime::Instance;
use serde::{Serialize, Deserialize};

use wasi_binio_shared_mods::{split_i64_to_i32, join_i32_to_i64};//We put shared utilities function into shared_mods. basicly i32 i64 convertor

//Host use this function to call wasm's reserve memory function to reserve a buffer inside
//wasm's linear memory
// value is the args will be transferred to wasm
// result is (buffer_pointer, buffer_length)
fn reserve_wasm_memory_buffer<'a, T> (value: &T, instance: &Instance ) -> Result<(i32, i32), &'a str> where T: Serialize {
    let buffer_size = bincode::serialized_size(value).expect("Error when calculate the buffer size using bincode") as i32; 
    let prepare_buffer_func = instance
        .get_export("prepare_buffer")
        .and_then(|e| e.func())
        .expect("Cannot get exported function 'prepare_buffer' from wasm instance")
        .get1::<i32, i64>().expect("Cannot get exported function 'prepare_buffer' from wasm instance");
    
    let result = prepare_buffer_func(buffer_size).expect("Error in prepare_buffer_func");
    Ok(shared_mods::split_i64_to_i32(result))
}

fn fill_buffer<T> (value: &T, instance: &Instance, ptr:i32, len:i32) -> Result<(), &'static str> where T: Serialize {
    let mem = instance.get_export("memory").expect("Cannot get export memory from instance").memory().expect("cannot get memory");
    let mem_array: &mut [u8];
    let serialized_array = bincode::serialize(value).expect("Error inside bincode::serialize");
    unsafe{
        mem_array = mem.data_unchecked_mut();
        for i in 0..len {
            mem_array[ptr as usize + i as usize] = serialized_array[i as usize];
        }
    }
    Ok(())
}

/// From host, call a wasm function with complex arguments and results.
/// 
// --snip--
/// 
///
/// # Examples
///
/// ```
/// let instance = Instance::new(&module, &imports).unwrap();
///
///    //We create two Point values here
///    let point1 = Point{x:2, y:3};
///    let point2 = Point{x:8, y:9};
///    
///    println!("Host app created two Point values which will be sent to the wasm module: {:?} {:?}", point1, point2);
///
///    //we use binio function call_stub to indirectly call the 'do_compute' function in 
///    //our wasm module.
///    //Because wasm function cannot take (point1, point2) as input args so 
///    //we have to indirectly using call_stub
///    //call_stub takes the do_compute's args as its args, then returns whatever
///    //do_compute will return
///    let result: Rect/* It is very important to have the Type here */
///        = call_stub(&instance, //Instance is needed to find export wasm function
///            &(point1, point2),//This was originally 'do_compute' args, now used by call_stub, it will then be transferred to wasm
///            "do_compute"//this is the name of the user function in wasm.
///        );
///           
/// ```
pub fn call_stub <'a, T, R> (instance: &'a Instance, arg: &T, func_name: &str) -> R 
where T: Serialize, R: Deserialize<'a> {
    
    let (arg_buffer_ptr, arg_buffer_len) = reserve_wasm_memory_buffer(arg, instance).expect("Error inside reserve_wasm_memory_buffer");
    fill_buffer(&arg, instance, arg_buffer_ptr, arg_buffer_len).expect("fill_buffer has error");
    let wasm_func = instance
        .get_export(func_name)
        .and_then(|e| e.func())
        .expect(&format!("call_stub cannot get exported function name: {} from wasm instance. Please make sure you exported such a function", func_name))
        .get2::<i32, i32, i64>().expect(&format!("call_stub cannot get correct FuncType for function name: {} from wasm instance. Please make sure you exported such a function", func_name));

    let result_in_i64 = wasm_func(arg_buffer_ptr, arg_buffer_len).expect("do_compute error"); //TODO, handle error
    let (result_buffer_ptr, result_buffer_len) = shared_mods::split_i64_to_i32(result_in_i64);
    let mem = instance.get_export("memory").expect("Error exporting memory from instance").memory().expect("memory fail");
    let mem_array_ref = unsafe {mem.data_unchecked()};

    let start_ptr = mem_array_ref.as_ptr().wrapping_offset(result_buffer_ptr as isize);
    let a = unsafe{std::slice::from_raw_parts(start_ptr, result_buffer_len as usize)};
    bincode::deserialize(a).expect("deseralize error")
}

