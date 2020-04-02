# Hello example
## Gist

### Make a Rectangle around two Points

There are two projects in the `hello` example. The hello-host and hello-wasm.

Hello-host is running in the wasmtime host app. It create two `Point` values. Use a `do_compute` function inside the Hello-wasm module to calculate the minimize Rectangle to hold around those two points.

The Hello-wasm module takes two input `Point` value and return the `Rantangle` value back to the host.

The `Point` and `Rect` data structure:

```
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}
```

### Challenges

This is a simple task however it is pretty hard using wasm at this moment. That's because wasm can only support i32, i64 f32 f64 four data types. The Point and Rect are complex data types that we cannot just send as function parameters like

` let rect: Rect = do_compute(point1, point2) .....`

So we have to make a utility tool to convert those complex structured to byte array and store in the wasm Memory then transfer the buffer pointer and length between host and wasm. Here comes the Binio.

### Solution
Use `Serde` and `Bincode` to serialize and deserialize the parameters and result to the memory shared between wasm and host. In order to make other developers easy to use, we made the `Binio` crate.

The usage would be as simple as 


```
//Cargo.toml for the host

[dependencies]
binio-host = { path ="../../binio-host"}

```


```
//Hello-host

let instance = Instance::new(&module, &imports).unwrap();
    let result: Rect= call_stub(&instance, &(point1, point2), "do_compute");
    println!("return Rect {:?}", result);
```

```
//Cargo.toml for the wasm

[dependencies]
binio-wasm = { path ="../../binio-wasm"
```

```
//Hello-wasm

use hello_shared::{Point, Rect};
use binio_wasm;
#[no_mangle]
fn prepare_buffer(buffer_size: i32)->i64 {
    binio_wasm::wasm_prepare_buffer(buffer_size)
}

#[no_mangle]
fn do_compute(ptr:i32, buffer_size: i32)->i64{
    let point_tuple : (Point, Point) = binio_wasm::wasm_deserialize(ptr, buffer_size);
    println!("Log from wasm -- point1 is {:?}", point_tuple.0);
    println!("Log from wasm -- point2 is {:?}", point_tuple.1);

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
    binio_wasm::wasm_serialize(&rect)
}
```

# Build
For host apps
` cargo run`
For wasm modules
`cargo wasi build --release`

Please make sure you have `wasmtime` and `wasi` installed first.

