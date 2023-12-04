// Copyright (c) ZeroC, Inc.

use crate::decoder::{DecodeResult, Decoder};
use crate::Encoding;

/// TODO
pub trait TryDecode<E: Encoding>
    where Self: Sized,
{
    /// TODO
    fn try_decode(decoder: &mut Decoder<E>) -> DecodeResult<Self>;

    /// TODO
    fn decode(decoder: &mut Decoder<E>) -> Self {
        default_error_handler(Self::try_decode(decoder))
    }
}

/// TODO
pub trait TryDecodeCollection<E: Encoding, T>
    where Self: Sized,
{
    /// TODO
    fn try_decode_with_fn(decoder: &mut Decoder<E>, decode_fn: DecodeFn<T, E>) -> DecodeResult<Self>;

    /// TODO
    fn decode_with_fn(decoder: &mut Decoder<E>, decode_fn: DecodeFn<T, E>) -> Self {
        default_error_handler(Self::try_decode_with_fn(decoder, decode_fn))
    }
}

/// TODO
pub type DecodeFn<T, E> = fn(&mut Decoder<E>) -> DecodeResult<T>;

#[inline(always)]
/// TODO
fn default_error_handler<T>(result: DecodeResult<T>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("failed to decode\n{error:?}"),
    }
}
