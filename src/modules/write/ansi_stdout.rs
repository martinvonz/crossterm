//! This is an ANSI specific implementation for the screen write
//! This module is used for windows 10 terminals and unix terminals by default.
//! This module uses the stdout to write to the console.

use super::IStdout;

use std::any::Any;
use std::cell::RefCell;
use std::sync::{Arc,Mutex};
use std::io::{self, Read, Write,Stdout, stdout};
use std::str::from_utf8;

/// This struct is an ANSI escape code implementation for screen related actions.
pub struct AnsiStdout {
    pub handle: Stdout,
}

impl IStdout for AnsiStdout {
   fn write_str(&self, string: &str) -> io::Result<usize> {
       let out = &self.handle;
       let mut handle = out.lock();
        write!(handle, "{}", string)?;
       handle.flush();
        Ok(0)
    }

    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let out = &self.handle;
        let mut handle = out.lock();
        handle.write(buf)?;
        Ok(0)
    }

    fn flush(&self) -> io::Result<()> {
        let out = &self.handle;
        let mut handle = out.lock();
        handle.flush();

        Ok(())
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

impl AnsiStdout {
    pub fn new() -> Self {
        AnsiStdout { handle: stdout() }
    }
}