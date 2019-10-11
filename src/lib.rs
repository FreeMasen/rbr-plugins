//! rbr-plugin-runner (lib)
//! step5
pub use rbr_macro::rbr_macro;

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