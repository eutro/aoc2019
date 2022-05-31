#[cfg(not(feature = "wasm"))]
pub use std::io::*;
#[cfg(not(feature = "wasm"))]
pub use std::{println, print};

#[cfg(feature = "wasm")]
pub use wasm_io::*;
#[cfg(feature = "wasm")]
pub(crate) use wasm_io::{println, print};

#[allow(unused)]
#[macro_use]
mod wasm_io {
    pub static mut SIDE_EFFECT: usize = 0;

    #[no_mangle]
    #[inline(never)]
    pub extern "C" fn stdin_read_byte() -> i32 {
        // no-op, to replace
        unsafe {
            SIDE_EFFECT += 1;
        }
        -1
    }

    #[no_mangle]
    #[inline(never)]
    pub extern "C" fn stdout_write_byte(byte: i32) {
        // no-op, to replace
        unsafe {
            SIDE_EFFECT += 1;
        }
    }

    use std::fmt::Debug;
    pub use std::io::{Read, Write, BufRead, Error};
    use std::io::BufReader;

    macro_rules! iprintln {
        ($($arg:tt)*) => {
            writeln!($crate::io::stdout(), $($arg)*).unwrap()
        };
    }

    macro_rules! iprint {
        ($($arg:tt)*) => {
            write!($crate::io::stdout(), $($arg)*).unwrap()
        };
    }

    pub(crate) use iprintln as println;
    pub(crate) use iprint as print;

    pub struct Stdin;
    pub struct Stdout;
    pub type StdinLock = BufReader<Stdin>;

    pub fn stdin() -> Stdin {
        Stdin
    }

    pub fn stdout() -> Stdout {
        Stdout
    }

    fn stdin_bytes() -> impl Iterator<Item = u8> {
        std::iter::from_fn(|| {
            match stdin_read_byte() {
                -1 => None,
                b => Some(b as u8),
            }
        })
    }

    impl Read for Stdin {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let mut len = 0;
            for (i, o) in stdin_bytes().take(buf.len()).zip(buf.iter_mut()) {
                *o = i;
                len += 1;
            }
            Ok(len)
        }
    }

    impl Stdin {
        pub fn lock(&self) -> StdinLock {
            BufReader::new(Stdin)
        }

        pub fn read_line(&self, str: &mut String) -> Result<usize, Error> {
            self.lock().read_line(str)
        }

        pub fn lines(&self) -> impl Iterator<Item = Result<String, Error>> {
            self.lock().lines()
        }
    }

    impl Write for Stdout {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            for &b in buf {
                stdout_write_byte(b as i32);
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Stdout {
        pub fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()> {
            Write::write_fmt(self, fmt)
        }
    }
}
