//! rbr-plugin
//! step6
//! Plugin Code will go here

use rbr_plugin_runner::*;

/// What we would write if this was just
/// a rust library
#[rbr_macro]
pub fn double(t: Thing) -> Thing {
    Thing {
        stuff: t.stuff.repeat(2),
        times: t.times * 2
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}
