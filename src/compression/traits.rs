use std::io::{Read, Write};

use super::error::CompressionError;

pub trait Compressor {
    fn compress<R: Read, W: Write>(&self, reader: R, writer: W) -> Result<(), CompressionError>;
}

pub trait Decompressor {
    fn decompress<R: Read, W: Write>(&self, reader: R, writer: W) -> Result<(), CompressionError>;
}
