// Copyright (c) ZeroC, Inc.

use crate::Encoding;

/// TODO
#[derive(Debug)]
pub struct Encoder<'a, E: Encoding> {
    /// Which version of the Slice encoding this encoder is using.
    encoding: E,

    buffer_temp: core::marker::PhantomData<&'a ()>, // TODO
}

impl<'a, E: Encoding> Encoder<'a, E> {
    // TODO new function here?

    /// TODO
    pub fn write_byte(&mut self, data: u8) -> EncodeResult<()> {
        todo!() // TODO
    }

    /// TODO
    pub fn write_bytes(&mut self, data: &[u8]) -> EncodeResult<()> {
        todo!() // TODO
    }

    /// TODO
    pub fn reserve(&mut self, count: usize) -> EncodeResult<&'a mut [u8]> {
        todo!() // TODO
    }
}

/// A specialized [`Result`] type for encoding functions which may produce an error.
///
/// It is a direct mapping to [`std::result::Result`] with an `Err` type of [`EncodeError`].
pub type EncodeResult<T> = Result<T, EncodeError>;

/// TODO
#[derive(Debug)]
pub enum EncodeError {

}
