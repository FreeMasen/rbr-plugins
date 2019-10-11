//! rbr-plugin
//! step3
//! Plugin Code will go here

/// What we would write if this was just
/// a rust library
pub fn double(s: String) -> String {
    s.repeat(2)
}

/// What is required to get around wasm's
/// numbers only implementation
#[no_mangle]
pub fn double_(ptr: u32, len: u32) -> u32 {
    let bytes = unsafe {
        ::std::slice::from_raw_parts(ptr as _, len as _)
    };
    let mut s = String::from_utf8(bytes.to_vec()).unwrap();
    s = double(s);
    let ret_len = s.as_bytes().len();
    unsafe {
        ::std::ptr::write(1 as _, ret_len as u32);
    }
    s.as_ptr() as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}
