# Hello example
## Gist

### Make a Reactangle around two Points

There are two projects in the `hello` example. The hello-host and hello-wasm.

Hello-host is running in the wasmtime host app. It create two `Point` values. Use a `do_compute` function inside the Hello-wasm module to calculate the minimize Reactangle to hold around those two points.

The Hello-wasm module takes two input `Point` value and return the `Rantangle` value back to the host.

The `Point` and `React` data structure:

https://github.com/TeaRUST/Binio/blob/dc742034b1cc63dde57547d13cd06760dcca7543/examples/hello-shared/src/lib.rs#L3-L14

### Challenges

This is a simple task however it is pretty hard using wasm at this moment. That's because wasm can only support i32, i64 f32 f64 four data types. The Point and React are complex data types that we cannot just send as function parameters like

` let rect: Rect = do_compute(point1, point2) .....`

So we have to make a utility tool to convert those complex structured to byte array and store in the wasm Memory then transfer the buffer pointer and length between host and wasm. This comes to Binio.

