
use bincode;
use serde::{Serialize, Deserialize};
use shared_mods;

static mut BUFFERS : Vec<Vec<u8>> = Vec::new();

pub fn wasm_prepare_buffer(size: i32) -> i64 {
    let buffer : Vec<u8> = Vec::with_capacity(size as usize);
    let ptr = buffer.as_ptr() as i32;
    unsafe{BUFFERS.push(buffer)};
    shared_mods::join_i32_to_i64(ptr, size )
}

pub fn wasm_deserialize<'a, T>(offset:i32, size:i32)->T where T: Deserialize<'a> {
    let slice = unsafe { std::slice::from_raw_parts(offset as *const _, size as usize) };
    let _buffer_would_be_dropped = unsafe{BUFFERS.pop()};
    bincode::deserialize(slice).unwrap()
}

pub fn wasm_serialize<'a, T>(value: &T)->i64 where T: Serialize {
    let buffer_size = bincode::serialized_size(value).unwrap() as i32; 
    let (result_ptr, result_len) = shared_mods::split_i64_to_i32(wasm_prepare_buffer(buffer_size));
    let serialized_array = bincode::serialize(value).unwrap();
    let slice = unsafe { std::slice::from_raw_parts_mut(result_ptr as *mut _, result_len as usize)};
    for i in 0..result_len {
        slice[i as usize] = serialized_array[i as usize];
    }
    shared_mods::join_i32_to_i64(result_ptr, result_len)
}