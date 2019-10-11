//! rbr-plugin-runner (lib)
//! step6
pub use rbr_macro::rbr_macro;
pub use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
extern "C" {
    pub fn print_str(ptr: *const u8, len: usize);
}

pub fn setup_error_handler() {
    fn err_hook(info: &::std::panic::PanicInfo) {
        let msg = info.to_string();
        unsafe {
            print_str(msg.as_ptr(), msg.len());
        }
    }
    ::std::sync::Once::new().call_once(|| {
        ::std::panic::set_hook(Box::new(err_hook))
    });
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thing {
    pub stuff: String,
    pub times: u32,
}