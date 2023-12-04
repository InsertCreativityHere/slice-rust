// Copyright (c) ZeroC, Inc.

// These modules are private because they don't export any types, just implementations.
mod decoding;
mod encoding;

use crate::decoder::{DecodeError, DecodeResult, Decoder};
use crate::encoder::{Encoder, EncodeResult};
use crate::Encoding;

// TODO
pub const VARINT32_MIN: i32 = i32::MIN;
// TODO
pub const VARINT32_MAX: i32 = i32::MAX;
// TODO
pub const VARUINT32_MIN: u32 = u32::MIN;
// TODO
pub const VARUINT32_MAX: u32 = u32::MAX;
// TODO
pub const VARINT62_MIN: i64 = i64::MIN >> 2;
// TODO
pub const VARINT62_MAX: i64 = i64::MAX >> 2;
// TODO
pub const VARUINT62_MIN: u64 = u64::MIN >> 2;
// TODO
pub const VARUINT62_MAX: u64 = u64::MAX >> 2;

/// TODO
type Slice2Decoder<'a> = Decoder<'a, Slice2>;

/// TODO
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slice2;

impl Encoding for Slice2 {
    /// TODO
    fn try_decode_size(decoder: &mut Slice2Decoder) -> DecodeResult<usize> {
        decoder.try_decode_varuint62().and_then(|uint64| {
            uint64.try_into().map_err(|_| {
                DecodeError::InvalidData {
                    desc: "TODO",
                }
            })
        })
    }

    /// TODO
    fn try_encode_size(size: usize, encoder: &mut Encoder<Slice2>) -> EncodeResult<()> {
        todo!() // TODO
    }
}
