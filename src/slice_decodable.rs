
use crate::bit_sequence::BitSequenceReader;
use crate::slice_decoder::{DecodeError, DecodeResult, Slice2Decoder, SliceDecoder};
use crate::varints::{VarInt32, VarUInt32, VarInt62, VarUInt62};
use core::hint::unreachable_unchecked;


#[derive(Debug)]
pub enum DecodeError {
    OutOfRange { value: i128, min: i128, max: i128, typename: &'static str },
}


#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::hash::Hash;

// =============================================================================
// Sequence type implementations
// =============================================================================

#[cfg(feature = "alloc")]
impl<T> SliceDecodableCollection<T> for Vec<T> {
    fn try_decode_with_fn(decoder: &mut SliceDecoder, decode_element: DecodeFn<T>) -> DecodeResult<Self> {
        let length = *VarUInt62::try_decode(decoder)?;
        let mut vector = Vec::with_capacity(length as usize);                                       // TODO can be bad

        for _ in 0..length {
            let value = decode_element(decoder)?;
            vector.push(value);
        }
        Ok(vector)
    }
}

#[cfg(feature = "alloc")]
impl<T> SliceDecodable for Vec<T>
    where T: SliceDecodable,
{
    fn try_decode(decoder: &mut SliceDecoder) -> DecodeResult<Self> {
        Self::try_decode_with_fn(decoder, <T>::try_decode)
    }
}

#[cfg(feature = "alloc")]
impl<T> SliceDecodableCollection<T> for Vec<Option<T>> {
    fn try_decode_with_fn(decoder: &mut SliceDecoder, decode_element: DecodeFn<T>) -> DecodeResult<Self> {
        let length = *VarUInt62::try_decode(decoder)?;
        let mut vector = Vec::with_capacity(length as usize);                                       // TODO can be bad

        let bit_sequence_size = length.div_ceil(8) as usize;                                        // TODO this will silently wrap for lengths > usize::MAX. Maybe we should do something more safe?
        let (bit_sequence_buffer, mut element_decoder) = decoder.split_at(bit_sequence_size)?;
        let mut bit_sequence_reader = BitSequenceReader::new(bit_sequence_buffer);

        for _ in 0..length {
            let value = match bit_sequence_reader.read_bit() {
                true => Some(decode_element(&mut element_decoder)?),
                false => None,
            };
            vector.push(value);
        }
        Ok(vector)
    }
}

#[cfg(feature = "alloc")]
impl<T> SliceDecodable for Vec<Option<T>>
    where T: SliceDecodable,
{
    fn try_decode(decoder: &mut SliceDecoder) -> DecodeResult<Self> {
        Self::try_decode_with_fn(decoder, <T>::try_decode)
    }
}

// =============================================================================
// Dictionary type implementations
// =============================================================================

macro_rules! try_decode_dictionary_body {
    () => {
        fn try_decode(decoder: &mut SliceDecoder) -> DecodeResult<Self> {
            Self::try_decode_with_fn(decoder, |decoder| {
                let key = K::try_decode(decoder)?;
                let value = V::try_decode(decoder)?;
                Ok((key, value))
            })
        }
    }
}

macro_rules! try_decode_optional_dictionary_body {
    () => {
        fn try_decode(decoder: &mut SliceDecoder) -> DecodeResult<Self> {
            Self::try_decode_with_fn(decoder, |decoder| {
                let is_set = bool::try_decode(decoder)?;
                let key = K::try_decode(decoder)?;
                let value = match is_set {
                    true => Some(V::try_decode(decoder)?),
                    false => None,
                };
                Ok((key, value))
            })
        }
    }
}


#[cfg(feature = "std")]
impl<K, V> SliceDecodableCollection<(K, V)> for HashMap<K, V>
    where K: Eq + Hash,
{
    fn try_decode_with_fn(decoder: &mut SliceDecoder, decode_element: DecodeFn<(K, V)>) -> DecodeResult<Self> {
        let length = *VarUInt62::try_decode(decoder)?;
        let mut hash_map = HashMap::with_capacity(length as usize);                                 // TODO can be bad

        for _ in 0..length {
            let (key, value) = decode_element(decoder)?;
            hash_map.insert(key, value);
        }
        Ok(hash_map)
    }
}

#[cfg(feature = "std")]
impl<K, V> SliceDecodable for HashMap<K, V>
where
    K: SliceDecodable + Eq + Hash,
    V: SliceDecodable,
{
    try_decode_dictionary_body!();
}

#[cfg(feature = "std")]
impl<K, V> SliceDecodable for HashMap<K, Option<V>>
where
    K: SliceDecodable + Eq + Hash,
    V: SliceDecodable,
{
    try_decode_optional_dictionary_body!();
}

#[cfg(feature = "alloc")]
impl<K, V> SliceDecodableCollection<(K, V)> for BTreeMap<K, V>
    where K: Ord,
{
    fn try_decode_with_fn(decoder: &mut SliceDecoder, decode_element: DecodeFn<(K, V)>) -> DecodeResult<Self> {
        let length = *VarUInt62::try_decode(decoder)?;
        let mut btree_map = BTreeMap::new();

        for _ in 0..length {
            let (key, value) = decode_element(decoder)?;
            btree_map.insert(key, value);
        }
        Ok(btree_map)
    }
}

#[cfg(feature = "alloc")]
impl<K, V> SliceDecodable for BTreeMap<K, V>
where
    K: SliceDecodable + Ord,
    V: SliceDecodable,
{
    try_decode_dictionary_body!();
}

#[cfg(feature = "alloc")]
impl<K, V> SliceDecodable for BTreeMap<K, Option<V>>
where
    K: SliceDecodable + Ord,
    V: SliceDecodable,
{
    try_decode_optional_dictionary_body!();
}
