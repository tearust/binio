// hello_shared contains Point and Rect struct which are the function args and result. 
// we will send and receive them using binio 
use hello_shared::{Point, Rect};

// binio_wasm is used inside wasm mobile instead of host app.
use wasi_binio_wasm;

// We have to have prepare_buffer function here under [no_mangle] because the binio-host will 
// look for this function from the wasm exports.
// the purpose of this function is to allocate memory for function args or result
// the return i64 actually contains two i32. One for buffer pointer another for buffer length
#[no_mangle]
fn prepare_buffer(buffer_size: i32)->i64 {
    wasi_binio_wasm::wasm_prepare_buffer(buffer_size)
}

// do_compute is the main function to do the Point to Rect computation
// It need to be exported so to be called from rumtime
#[no_mangle]
fn do_compute(ptr:i32, buffer_size: i32)->i64{
    //although we can only use i32 , i32 as function args, we can still call
    //binio_wasm::wasm_deserialize to get the Point struct from runtime. 
    let point_tuple : (Point, Point) = wasi_binio_wasm::wasm_deserialize(ptr, buffer_size).unwrap();
    
    //print out points make sure we have got the correct args
    println!("Log from wasm -- point1 is {:?}", point_tuple.0);
    println!("Log from wasm -- point2 is {:?}", point_tuple.1);

    //the user logic, not related to binio
    let (left, right) = {
        if point_tuple.0.x > point_tuple.1.x{
            (point_tuple.1.x, point_tuple.0.x)
        }
        else{
            (point_tuple.0.x, point_tuple.1.x)
        }
    };

    let (top, bottom) = {
        if point_tuple.0.y > point_tuple.1.y {
            (point_tuple.1.y, point_tuple.0.y)
        }
        else{
            (point_tuple.0.y, point_tuple.1.y)
        }
    };
    let rect = Rect{left, right, top , bottom};
    //Now we have the rect as function's result. we cannot just return a Rect struct
    //becuase wasm support i32 i64 f32 f64 only
    // so we need to use binio to transfer the rect into memory buffer, then send back the address/lenght
    wasi_binio_wasm::wasm_serialize(&rect).unwrap()
}

