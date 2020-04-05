//! # wasi_binio_host and wasi_binio_wasm
//!
//! `wasi_binio_wasm` is the webassembly crate of wasm_binio. Another crate is `wasi_binio_host` which used in host.
//! wasi_binio_host creates a call_stub so that host can call a webassembly function with complex data structure arguments and results.
//! wasi_binio_wasm prepare the linear memory buffer and exports required wasm function so that the host can call.
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
    
use bincode;
use serde::{Serialize, Deserialize};
use wasi_binio_shared_mods::{join_i32_to_i64, split_i64_to_i32};//We put shared utilities function into shared_mods. basicly i32 i64 convertor

static mut BUFFERS : Vec<Box<[u8]>> = Vec::new();

/// Allocate buffer from wasm linear memory in order to let host to deserialize arguments into
/// # Example
/// We have to have prepare_buffer function here under [no_mangle] because the binio-host will 
/// look for this function from the wasm exports.
/// the purpose of this function is to allocate memory for function args or result
/// the return i64 actually contains two i32. One for buffer pointer another for buffer length
/// ```
///#[no_mangle]
///fn prepare_buffer(buffer_size: i32)->i64 {
///    wasi_binio_wasm::wasm_prepare_buffer(buffer_size)
///}
/// ```
pub fn wasm_prepare_buffer(size: i32) -> i64 {
    //let buffer : Vec<u8> = Vec::with_capacity(size as usize);
    let buffer = Vec::<u8>::with_capacity(size as usize).into_boxed_slice();
    let ptr = buffer.as_ptr() as i32;
    unsafe{BUFFERS.push(buffer)};
    join_i32_to_i64(ptr, size )
}
/// Main function in wasm will call this function to deserialize the arguments to the prepared 
/// 
/// linear memory buffer.
// --snip--
/// # Example
/// ```
/// #[no_mangle]
///fn do_compute(ptr:i32, buffer_size: i32)->i64{
///    //although we can only use i32 , i32 as function args, we can still call
///    //binio_wasm::wasm_deserialize to get the Point struct from runtime. 
///    let point_tuple : (Point, Point) = wasi_binio_wasm::wasm_deserialize(ptr, buffer_size).unwrap();
    
///    //print out points make sure we have got the correct args
///    println!("Log from wasm -- point1 is {:?}", point_tuple.0);
///    println!("Log from wasm -- point2 is {:?}", point_tuple.1);

///    //the user logic, not related to binio
///    let (left, right) = {
///        if point_tuple.0.x > point_tuple.1.x{
///            (point_tuple.1.x, point_tuple.0.x)
///        }
///        else{
///            (point_tuple.0.x, point_tuple.1.x)
///        }
///    };

///    let (top, bottom) = {
///        if point_tuple.0.y > point_tuple.1.y {
///            (point_tuple.1.y, point_tuple.0.y)
///        }
///        else{
///            (point_tuple.0.y, point_tuple.1.y)
///        }
///    };
///    let rect = Rect{left, right, top , bottom};
///    //Now we have the rect as function's result. we cannot just return a Rect struct
///    //becuase wasm support i32 i64 f32 f64 only
///    // so we need to use binio to transfer the rect into memory buffer, then send back the address/lenght
///    wasi_binio_wasm::wasm_serialize(&rect).unwrap()
///}
///
pub fn wasm_deserialize<'a, T>(offset:i32, size:i32)->bincode::Result<T> 
where T: Deserialize<'a> {
    let slice = unsafe { std::slice::from_raw_parts(offset as *const _, size as usize) };
    let _buffer_would_be_dropped = unsafe{BUFFERS.pop()};
    bincode::deserialize(slice)
}


/// Main function in wasm will call this function to deserialize the arguments to the prepared 
/// 
/// linear memory buffer.
// --snip--
/// # Example
/// ```
/// #[no_mangle]
/// fn do_compute(ptr:i32, buffer_size: i32)->i64{
///    //although we can only use i32 , i32 as function args, we can still call
///    //binio_wasm::wasm_deserialize to get the Point struct from runtime. 
///    let point_tuple : (Point, Point) = wasi_binio_wasm::wasm_deserialize(ptr, buffer_size).unwrap();
        
///    //print out points make sure we have got the correct args
///    println!("Log from wasm -- point1 is {:?}", point_tuple.0);
///    println!("Log from wasm -- point2 is {:?}", point_tuple.1);
    
///    //the user logic, not related to binio
///    let (left, right) = {
///        if point_tuple.0.x > point_tuple.1.x{
///            (point_tuple.1.x, point_tuple.0.x)
///        }
///        else{
///            (point_tuple.0.x, point_tuple.1.x)
///        }
///    };
    
///    let (top, bottom) = {
///        if point_tuple.0.y > point_tuple.1.y {
///            (point_tuple.1.y, point_tuple.0.y)
///        }
///        else{
///            (point_tuple.0.y, point_tuple.1.y)
///        }
///    };
///    let rect = Rect{left, right, top , bottom};
///    //Now we have the rect as function's result. we cannot just return a Rect struct
///    //becuase wasm support i32 i64 f32 f64 only
///    // so we need to use binio to transfer the rect into memory buffer, then send back the address/lenght
///    wasi_binio_wasm::wasm_serialize(&rect).unwrap()
///}
///```
pub fn wasm_serialize<'a, T>(value: &T)->bincode::Result<i64> where T: Serialize {
    let buffer_size = bincode::serialized_size(value).unwrap() as i32; 
    let (result_ptr, result_len) = split_i64_to_i32(wasm_prepare_buffer(buffer_size));
    let serialized_array = bincode::serialize(value).unwrap();
    let slice = unsafe { std::slice::from_raw_parts_mut(result_ptr as *mut _, result_len as usize)};
    for i in 0..result_len {
        slice[i as usize] = serialized_array[i as usize];
    }
    Ok(join_i32_to_i64(result_ptr, result_len))
}