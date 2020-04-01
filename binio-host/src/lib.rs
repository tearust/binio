use wasmtime::Instance;
use serde::{Serialize, Deserialize};

use shared_mods;

fn reserve_wasm_memory_buffer<T> (obj: &T, instance: &Instance ) -> (i32, i32) where T: Serialize {
    let buffer_size = bincode::serialized_size(obj).unwrap() as i32; 
    let prepare_buffer_func = instance
        .get_export("prepare_buffer")
        .and_then(|e| e.func())
        .unwrap()
        .get1::<i32, i64>().unwrap();
    
    let result = prepare_buffer_func(buffer_size).unwrap();
    shared_mods::split_i64_to_i32(result)
}

fn fill_buffer<T> (obj: &T, instance: &Instance, ptr:i32, len:i32) -> Result<(), &'static str> where T: Serialize {
    let mem = instance.get_export("memory").unwrap().memory().unwrap();
    let mem_array: &mut [u8];
    let serialized_array = bincode::serialize(obj).unwrap();
    unsafe{
        mem_array = mem.data_unchecked_mut();
        for i in 0..len {
            mem_array[ptr as usize + i as usize] = serialized_array[i as usize];
        }
    }
    Ok(())
}

pub fn call_stub <'a, T, R> (instance: &'a Instance, arg: &T, func_name: &str) -> R 
where T: Serialize, R: Deserialize<'a> {
    
    let (arg_buffer_ptr, arg_buffer_len) = reserve_wasm_memory_buffer(arg, instance);
    fill_buffer(&arg, instance, arg_buffer_ptr, arg_buffer_len).unwrap();
    let do_compute = instance
        .get_export(func_name)
        .and_then(|e| e.func())
        .unwrap()
        .get2::<i32, i32, i64>().unwrap();

    let result_in_i64 = do_compute(arg_buffer_ptr, arg_buffer_len).expect("do_compute error"); //TODO, handle error
    let (result_buffer_ptr, result_buffer_len) = shared_mods::split_i64_to_i32(result_in_i64);
    let mem = instance.get_export("memory").unwrap().memory().expect("memory fail");
    let mem_array_ref = unsafe {mem.data_unchecked()};

    let start_ptr = mem_array_ref.as_ptr().wrapping_offset(result_buffer_ptr as isize);
    let a = unsafe{std::slice::from_raw_parts(start_ptr, result_buffer_len as usize)};
    bincode::deserialize(a).expect("deseralize error")
}

