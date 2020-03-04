//! Asynchronous access to a bincode-encoded item stream.
//!
//! This crate enables you to asynchronously read from a bincode-encoded stream, or write
//! bincoded-encoded values. `bincode` does not support this natively, as it cannot easily [resume
//! from stream errors while encoding or decoding](https://github.com/TyOverby/bincode/issues/229).
//!
//! `async-bincode` works around that on the receive side by buffering received bytes until a full
//! element's worth of data has been received, and only then calling into bincode. To make this
//! work, it relies on the sender to prefix each encoded element with its encoded size.
//!
//! On the write side, `async-bincode` buffers the serialized values, and asynchronously sends the
//! resulting bytestream.
#![deny(missing_docs)]

mod reader;
mod stream;
mod writer;

pub use crate::reader::AsyncBincodeReader;
pub use crate::stream::AsyncBincodeStream;
pub use crate::writer::AsyncBincodeWriter;
pub use crate::writer::{AsyncDestination, BincodeWriterFor, SyncDestination};