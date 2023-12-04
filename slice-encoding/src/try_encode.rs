// Copyright (c) ZeroC, Inc.

use crate::encoder::{EncodeResult, Encoder};
use crate::Encoding;

/// TODO
pub trait TryEncode<E: Encoding>
    where Self: Sized,
{
    /// TODO
    fn try_encode(self, encoder: &mut Encoder<E>) -> EncodeResult<()>;

    /// TODO
    fn encode(self, encoder: &mut Encoder<E>) {
        default_error_handler(self.try_encode(encoder))
    }
}

/// TODO
pub trait TryEncodeCollection<E: Encoding, T>
    where Self: Sized,
{
    /// TODO
    fn try_encode_with_fn(self, encoder: &mut Encoder<E>, encode_fn: EncodeFn<T, E>) -> EncodeResult<()>;

    /// TODO
    fn encode_with_fn(self, encoder: &mut Encoder<E>, encode_fn: EncodeFn<T, E>) {
        default_error_handler(self.try_encode_with_fn(encoder, encode_fn))
    }
}

/// TODO
pub type EncodeFn<T, E> = fn(T, &mut Encoder<E>) -> EncodeResult<()>;

#[inline(always)]
/// TODO
fn default_error_handler<T>(result: EncodeResult<T>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("failed to encode\n{error:?}"),
    }
}
