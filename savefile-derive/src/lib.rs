#![feature(proc_macro)]
extern crate proc_macro;

#[proc_macro_derive(Savefile, attributes(versions, versions_as, default_val, default_fn))]
pub fn savefile(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic!("Not implemented")
}


