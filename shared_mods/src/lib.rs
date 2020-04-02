//! # wasi_binio_shared for wasi_binio_host and wasi_binio_wasm
//!
//! `wasi_binio_wasm` is the host crate of wasm_binio. Another crate is `wasi_binio_host` which used in host.
//! wasi_binio_host creates a call_stub so that host can call a webassembly function with complex data structure arguments and results.
//! wasi_binio_wasm prepare the linear memory buffer and exports required wasm function so that the host can call.
//! 
//! As of today, wasm function can only take i32 i64 f32 f64 types. If you have a function in wasm like this
//! In wasm: ` do_compute(point1: &Point, point2: &Point)->Rect {...}`
//! there is no way to call it directly from host.
//! With the help from wasm_binio, we can instead call the call_stub function and send arguments and results like this
//! 
//! 

/// Combain two i32 into one i64
pub fn join_i32_to_i64( a:i32, b:i32)->i64 {
  //((a as i64) << 32) | (b as i64)
  (((a as u64) << 32) | (b as u64) & 0x00000000ffffffff) as i64
}
/// Split one i64 into two i32
pub fn split_i64_to_i32( r: i64)->(i32,i32){
  ( (((r as u64) & 0xffffffff00000000) >> 32) as i32 , ((r as u64) & 0x00000000ffffffff) as i32)
}

#[cfg(test)]
mod test{
    
    use super::*;
    #[test]
    
    fn test_i64_i32_convertor(){
        let a = (i32::MAX, i32::MAX);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));

        let a = (i32::MIN, i32::MAX);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));

        let a = (i32::MAX, i32::MIN);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));
        let a = (i32::MIN, i32::MIN);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));
        let a = (23, -32);
        assert_eq!(a, split_i64_to_i32(join_i32_to_i64(a.0, a.1)));
    }
    
}