// Copyright (c) ZeroC, Inc.

//! TODO write a big doc comment explaining this crate.

#![no_std]

// If the 'std' feature is set, pull in `std` as an external crate.
// Note that the prelude is still disabled, so you'll need to use explicit paths for types provided by `std`.
#[cfg(feature = "std")]
extern crate std;

// If the 'alloc' feature is set, pull in `alloc` as an external crate.
#[cfg(feature = "alloc")]
extern crate alloc;

// Only include the `slice2` module if the corresponding feature is set.
#[cfg(feature = "slice2")]
pub mod slice2;

// Only include the `slice1` module if the corresponding feature is set.
#[cfg(feature = "slice1")]
pub mod slice1;

pub mod decoder;
pub mod encoder;
pub mod io_types;
pub mod try_decode;
pub mod try_encode;

// These modules are private because they don't export any types, just implementations.
mod decoding;
mod encoding;

use decoder::{Decoder, DecodeResult};
use encoder::{Encoder, EncodeResult};

use core::fmt::Debug;

/// TODO
pub trait Encoding: Clone + Copy + Debug + Default + PartialEq + Eq {
    /// TODO
    fn try_decode_size(decoder: &mut Decoder<Self>) -> DecodeResult<usize>;

    /// TODO
    fn try_encode_size(size: usize, encoder: &mut Encoder<Self>) -> EncodeResult<()>;
}
