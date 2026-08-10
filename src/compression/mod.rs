mod error;
mod macros;
mod traits;

pub mod algorithms;

pub use error::CompressionError;
pub use traits::{Compressor, Decompressor};
