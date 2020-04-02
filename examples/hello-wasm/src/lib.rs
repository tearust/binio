//use serde::{Serialize, Deserialize};
use hello_shared::{Point, React};
use binio_wasm;
#[no_mangle]
fn prepare_buffer(buffer_size: i32)->i64 {
    binio_wasm::wasm_prepare_buffer(buffer_size)
}

#[no_mangle]
fn do_compute(ptr:i32, buffer_size: i32)->i64{
    let point_tuple : (Point, Point) = binio_wasm::wasm_deserialize(ptr, buffer_size);
    println!("point1 is {:?}", point_tuple.0);
    println!("point2 is {:?}", point_tuple.1);

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
    let rect = React{left, right, top , bottom};
    binio_wasm::wasm_serialize(&rect)
}

