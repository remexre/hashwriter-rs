//! A `Write` that keeps track of the hash of the content being written.
#![warn(missing_docs)]

extern crate digest;

use std::io::{Result as IoResult, Write};

use digest::{FixedOutput, Input};
use digest::generic_array::GenericArray;

/// A Write that accumulates a hash as it goes.
pub struct HashWriter<H: Input + FixedOutput, W: Write> {
    hash: H,
    writer: W,
}

impl<H: Input + FixedOutput, W: Write> HashWriter<H, W> {
    /// Creates a `HashWriter`.
    pub fn new(hash: H, writer: W) -> HashWriter<H, W> {
        HashWriter { hash, writer }
    }

    /// Creates a `HashWriter`, using a default value for the hash.
    pub fn from_writer(writer: W) -> HashWriter<H, W>
    where
        H: Default,
    {
        let hash = H::default();
        HashWriter::new(hash, writer)
    }

    /// Gets the hash from the HashWriter.
    pub fn digest(self) -> GenericArray<u8, H::OutputSize> {
        self.hash.fixed_result()
    }
}

impl<H: Input + FixedOutput, W: Write> Write for HashWriter<H, W> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.hash.process(buf);
        self.writer.write(buf)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.writer.flush()
    }
}
