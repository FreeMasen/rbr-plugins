//! rbr-plugin
//! step2
//! Plugin Code will go here

#[no_mangle]
pub fn double(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}
