//! rbr-macro
//! step6

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{
    Item,
    ItemFn,
    parse,
};
use proc_macro2::{
    Ident,
    Span,
};
use quote::quote;

#[proc_macro_attribute]
pub fn rbr_macro(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let item = parse(tokens).expect("Invalid rust tokens");
    match item {
        Item::Fn(func) => handle_func(func),
        _ => panic!("rbr_macro can only be applied to functions")
    }
}
fn handle_func(func: ItemFn) -> TokenStream {
    if func.sig.inputs.len() != 1 {
        panic!("rbr_macro can only be applied to functions that take 1 argument");
    }
    let orig = func.sig.ident.clone();
    let new = Ident::new(&format!("{}_", orig), Span::call_site());
    let ret = quote! {
        #func
        #[no_mangle]
        fn #new(ptr: u32, len: u32) -> u32 {
            setup_error_handler();
            let bytes = unsafe {
                ::std::slice::from_raw_parts(ptr as _, len as _)
            };
            let mut s = deserialize(bytes).expect("Unable to deserialize bytes");
            s = #orig(s);
            let bytes = serialize(&s).expect("Unable to serialize updated value");
            let ret_len = bytes.len();
            unsafe {
                ::std::ptr::write(1 as _, ret_len as u32);
            }
            bytes.as_ptr() as u32
        }
    };
    ret.into()
}