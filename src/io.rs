#[cfg(not(feature = "wasm"))]
pub use std::io::*;

pub use std::print;
pub use std::println;

#[cfg(feature = "wasm")]
pub use wasm_io::*;

#[allow(unused)]
mod wasm_io {
    macro_rules! println {
        ($($arg:tt)*) => {
            writeln!($crate::io::stdout(), $($arg)*).unwrap();
        };
    }

    macro_rules! print {
        ($($arg:tt)*) => {
            write!($crate::io::stdout(), $($arg)*).unwrap();
        };
    }

    pub struct Stdin;
    pub struct Stdout;
    pub struct StdinLock;
    #[derive(Debug)]
    pub struct Error;

    pub struct Lines<'a, T: ?Sized>(&'a mut T);

    pub trait BufRead {
        fn read_line(&mut self, str: &mut String) -> Result<usize, Error>;

        fn lines(&mut self) -> Lines<'_, Self> {
            Lines(self)
        }
    }

    impl<'a, T> Iterator for Lines<'a, T>
    where
        T: BufRead,
    {
        type Item = Result<String, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            let mut line = String::new();
            match self.0.read_line(&mut line) {
                Ok(0) => None,
                Ok(_) => Some(Ok(line)),
                Err(e) => Some(Err(e)),
            }
        }
    }

    pub trait Write {
        fn flush(&self) -> Result<(), Error> {
            // no-op
            Ok(())
        }
    }

    pub fn stdin() -> Stdin {
        Stdin
    }

    pub fn stdout() -> Stdout {
        Stdout
    }

    impl Stdin {
        pub fn lock(&self) -> StdinLock {
            StdinLock
        }
    }

    #[no_mangle]
    pub fn stdin_read_line(buf: &mut String) -> Result<usize, Error> {
        // no-op
        Ok(0)
    }

    impl BufRead for Stdin {
        fn read_line(&mut self, str: &mut String) -> Result<usize, Error> {
            stdin_read_line(str)
        }
    }

    impl BufRead for StdinLock {
        fn read_line(&mut self, str: &mut String) -> Result<usize, Error> {
            stdin_read_line(str)
        }
    }

    impl Stdin {
        pub fn read_line(&self, str: &mut String) -> Result<usize, Error> {
            self.lock().read_line(str)
        }
    }

    impl Write for Stdout {}
}
