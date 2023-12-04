// Copyright (c) ZeroC, Inc.

use super::{Slice2, Slice2Decoder};
use crate::decoder::{DecodeError, DecodeResult, Decoder};
use crate::decoding::implement_slice_decodable_for_primitive_numeric_type;
use crate::try_decode::TryDecode;

// TODO ADD COMMENTS TO EVERYTHING IN THIS FILE. RIGHT NOW WE'RE JUST GETTING IT FOR JOE!

// =============================================================================
// Fixed-length type implementations
// =============================================================================

impl TryDecode<Slice2> for i8 {
    fn try_decode(decoder: &mut Slice2Decoder) -> DecodeResult<Self> {
        let byte = decoder.read_byte()?;
        Ok(*byte as i8)
    }
}

implement_slice_decodable_for_primitive_numeric_type!(u16, "TODO", Slice2);
implement_slice_decodable_for_primitive_numeric_type!(u32, "TODO", Slice2);
implement_slice_decodable_for_primitive_numeric_type!(u64, "TODO", Slice2);

// =============================================================================
// Variable-length integer type implementations
// =============================================================================

impl Slice2Decoder<'_> {
    pub fn try_decode_varint32(&mut self) -> DecodeResult<i32> {
        let varint62 = self.try_decode_varint62()?;

        varint62.try_into().map_err(|_| DecodeError::OutOfRange {
            value: varint62 as i128,
            min: super::VARINT32_MIN as i128,
            max: super::VARINT32_MAX as i128,
            typename: "varint32",
        })
    }

    pub fn try_decode_varuint32(&mut self) -> DecodeResult<u32> {
        let varuint62 = self.try_decode_varuint62()?;

        varuint62.try_into().map_err(|_| DecodeError::OutOfRange {
            value: varuint62 as i128,
            min: super::VARUINT32_MIN as i128,
            max: super::VARUINT32_MAX as i128,
            typename: "varuint32",
        })
    }

    pub fn try_decode_varint62(&mut self) -> DecodeResult<i64> {
        let Some(size_prefix_byte) = self.peek_byte() else {
            return Err(DecodeError::EndOfBuffer)
        };

        let value = match size_prefix_byte & 0b11 {
            0b00 =>  i8::try_decode(self)? as i64,
            0b01 => i16::try_decode(self)? as i64,
            0b10 => i32::try_decode(self)? as i64,
            0b11 => i64::try_decode(self)?,
            _ => unsafe {
                std::hint::unreachable_unchecked()
            }
        };
        Ok(value >> 2)
    }

    pub fn try_decode_varuint62(&mut self) -> DecodeResult<u64> {
        let Some(size_prefix_byte) = self.peek_byte() else {
            return Err(DecodeError::EndOfBuffer)
        };

        let value = match size_prefix_byte & 0b11 {
            0b00 =>  u8::try_decode(self)? as u64,
            0b01 => u16::try_decode(self)? as u64,
            0b10 => u32::try_decode(self)? as u64,
            0b11 => u64::try_decode(self)?,
            _ => unsafe {
                std::hint::unreachable_unchecked()
            }
        };
        Ok(value >> 2)
    }
}
