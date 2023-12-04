// Copyright (c) ZeroC, Inc.

// These modules are private because they don't export any types, just implementations.
mod decoding;
mod encoding;

use crate::decoder::{Decoder, DecodeResult};
use crate::encoder::{Encoder, EncodeResult};
use crate::Encoding;

/// TODO
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slice1;

impl Encoding for Slice1 {
    /// TODO
    fn try_decode_size(decoder: &mut Decoder<Slice1>) -> DecodeResult<usize> {
        todo!() // TODO
    }

    /// TODO
    fn try_encode_size(size: usize, encoder: &mut Encoder<Slice1>) -> EncodeResult<()> {
        todo!() // TODO
    }
}
