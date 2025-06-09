#![feature(concat_idents, proc_macro_hygiene, repr_simd, simd_ffi, seek_stream_len)]
#![allow(static_mut_refs)]

#[cfg(feature = "skyline-web")]
extern crate skyline_web;

mod fighters;

#[no_mangle]
pub fn smashline_install() {
    fighters::install();
}

#[skyline::main(name = "ssbexo")]
pub fn main() {
    fighters::install();
    std::panic::set_hook(Box::new(|info: &std::panic::PanicHookInfo<'_>| {
        let location: &std::panic::Location<'_> = info.location().unwrap();
        let message: &str = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>"
            }
        };
        skyline::error::show_error(
            69,
            "Super Smash Bros: EXO has panicked! Please open the details and send a screenshot to PhazoGanon on Discord, and explain what you were doing.\0",
            &format!("Super Smash Bros: EXO has panicked with \"{message}\"\n\n{location}\0")
        );
    }));
}