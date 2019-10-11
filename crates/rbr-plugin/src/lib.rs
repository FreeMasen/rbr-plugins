//! rbr-plugin
//! step4
//! Plugin Code will go here

use rbr_macro::rbr_macro;

/// What we would write if this was just
/// a rust library
#[rbr_macro]
pub fn double(s: String) -> String {
    s.repeat(2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}
