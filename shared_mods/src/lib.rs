pub fn join_i32_to_i64( a:i32, b:i32)->i64 {
  //((a as i64) << 32) | (b as i64)
  (((a as u64) << 32) | (b as u64) & 0x00000000ffffffff) as i64
}

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