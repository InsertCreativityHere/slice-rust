// Copyright (c) ZeroC, Inc.

use crate::decoder::{DecodeError, DecodeResult, Decoder};
use crate::Encoding;
use crate::try_decode::TryDecode;

// TODO ADD COMMENTS TO THIS ENTIRE FILE! WE'RE JUST GETTING IT READY FOR JOE.

#[cfg(feature = "alloc")]
use alloc::string::String;

// =============================================================================
// Fixed-length type implementations
// =============================================================================

impl<E: Encoding> TryDecode<E> for bool {
    fn try_decode(decoder: &mut Decoder<E>) -> DecodeResult<Self> {
        let byte = *decoder.read_byte()?;

        // We strictly enforce the Slice spec; A bool must be encoded as either `0` or `1`.
        match byte {
            0 | 1 => Ok(byte != 0),
            _ => Err(DecodeError::IllegalValue {
                value: byte as i128,
                desc: "bools can only have a numeric value of `0` or `1`",
            })
        }
    }
}

impl<E: Encoding> TryDecode<E> for u8 {
    fn try_decode(decoder: &mut Decoder<E>) -> DecodeResult<Self> {
        decoder.read_byte().copied()
    }
}

macro_rules! implement_slice_decodable_for_primitive_numeric_type {
    ($ty:ty, $doc_text:literal, $encoding:ident$(: $($bounds:tt)+)?) => {
        impl$(<$encoding: $($bounds)+>)? TryDecode<$encoding> for $ty {
            #[doc = $doc_text]
            fn try_decode(decoder: &mut Decoder<$encoding>) -> DecodeResult<Self> {
                let bytes = decoder.read_array_exact()?;
                Ok(<$ty>::from_le_bytes(*bytes))
            }
        }
    }
}

pub(crate) use implement_slice_decodable_for_primitive_numeric_type;

implement_slice_decodable_for_primitive_numeric_type!(i16, "TODO", E: Encoding);
implement_slice_decodable_for_primitive_numeric_type!(i32, "TODO", E: Encoding);
implement_slice_decodable_for_primitive_numeric_type!(i64, "TODO", E: Encoding);
implement_slice_decodable_for_primitive_numeric_type!(f32, "TODO", E: Encoding);
implement_slice_decodable_for_primitive_numeric_type!(f64, "TODO", E: Encoding);

// =============================================================================
// Sequence type implementations
// =============================================================================

#[cfg(feature = "alloc")]
impl<E: Encoding> TryDecode<E> for String {
    fn try_decode(decoder: &mut Decoder<E>) -> DecodeResult<Self> {
        let length = E::try_decode_size(decoder)?;
        let buffer = decoder.read_bytes_exact(length as usize)?.to_vec();                                 // TODO can be bad

        String::from_utf8(buffer).map_err(|_| DecodeError::InvalidData {
            desc: "encountered invalid utf-8 while decoding string",
        })
    }
}
