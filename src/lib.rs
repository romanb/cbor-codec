// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(box_patterns, num_bits_bytes)]

extern crate byteorder;

pub mod types;
pub mod values;
pub mod decoder;
pub mod encoder;

#[cfg(test)]
extern crate rustc_serialize;

#[cfg(feature="arbitraries")]
extern crate quickcheck;

#[cfg(feature="arbitraries")]
extern crate rand;

#[cfg(feature="arbitraries")]
pub mod arbitraries;
