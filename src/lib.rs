//! Set panic behavior to abort
//!
//! This is an implementation of `panic_fmt` that simply calls [`intrinsics::abort`].
//!
//! [`intrinsics::abort`]: https://doc.rust-lang.org/core/intrinsics/fn.abort.html

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![no_std]

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(
    _args: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _col: u32,
) -> ! {
    core::intrinsics::abort()
}
