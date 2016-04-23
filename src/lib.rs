//! An implementation of [FLAC](https://xiph.org/flac), free lossless audio
//! codec, written in Rust.
//!
//! The code is available on [GitHub](https://github.com/sourrust/flac).
//!
//! # Examples
//!
//! Basic decoding from a file.
//!
//! ```
//! use flac::StreamReader;
//! use std::fs::File;
//!
//! match StreamReader::<File>::from_file("path/to/file.flac") {
//!   Ok(mut stream) => {
//!     // Copy of `StreamInfo` to help convert to a different audio format.
//!     let info = stream.info();
//!
//!     for sample in stream.iter::<i16>() {
//!       // Iterate over each decoded sample
//!     }
//!   }
//!   Err(error)     => println!("{:?}", error),
//! }
//! ```

#[macro_use]
extern crate nom;

#[macro_use]
mod utility;
mod frame;
mod subframe;
pub mod metadata;
pub mod stream;

pub use metadata::Metadata;
pub use stream::{Stream, StreamBuffer, StreamReader, StreamIter};
pub use utility::{
  Sample, SampleSize,
  StreamProducer, ReadStream, ByteStream,
  ErrorKind
};
