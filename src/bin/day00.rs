use std::io::Write;
use crate::io;

#[no_mangle]
pub fn day_00() {
    io::stdout().write(b"Hello, world!").unwrap();
}
