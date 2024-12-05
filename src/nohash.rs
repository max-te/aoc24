// Modified from https://github.com/solana-labs/nohash-hasher/blob/master/src/lib.rs
// Original license below
//
// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 or MIT license, at your option.
//
// A copy of the Apache License, Version 2.0 is included in the software as
// LICENSE-APACHE and a copy of the MIT license is included in the software
// as LICENSE-MIT. You may also obtain a copy of the Apache License, Version 2.0
// at https://www.apache.org/licenses/LICENSE-2.0 and a copy of the MIT license
// at https://opensource.org/licenses/MIT.

use core::{
    fmt,
    hash::{BuildHasherDefault, Hasher},
    marker::PhantomData,
};

pub type BuildTailHasher<T> = BuildHasherDefault<TailHasher<T>>;

pub struct TailHasher<T>(u64, PhantomData<T>);

impl<T> fmt::Debug for TailHasher<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("TailHasher").field(&self.0).finish()
    }
}

impl<T> Default for TailHasher<T> {
    fn default() -> Self {
        TailHasher(0, PhantomData)
    }
}

impl<T> Clone for TailHasher<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for TailHasher<T> {}

pub trait IsEnabled {}

impl IsEnabled for u8 {}
impl IsEnabled for u16 {}
impl IsEnabled for u32 {}
impl IsEnabled for u64 {}
impl IsEnabled for usize {}
impl IsEnabled for i8 {}
impl IsEnabled for i16 {}
impl IsEnabled for i32 {}
impl IsEnabled for i64 {}
impl IsEnabled for isize {}

impl<T: IsEnabled> Hasher for TailHasher<T> {
    fn write(&mut self, _: &[u8]) {
        panic!("Invalid use of TailHasher")
    }

    fn write_u8(&mut self, n: u8) {
        self.0 = self.0 << 8 | u64::from(n)
    }
    fn write_u16(&mut self, n: u16) {
        self.0 = self.0 << 16 | u64::from(n)
    }
    fn write_u32(&mut self, n: u32) {
        self.0 = self.0 << 32 | u64::from(n)
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = n
    }
    fn write_usize(&mut self, n: usize) {
        self.0 = n as u64
    }

    fn write_i8(&mut self, n: i8) {
        self.0 = self.0 << 8 | n as u64
    }
    fn write_i16(&mut self, n: i16) {
        self.0 = self.0 << 16 | n as u64
    }
    fn write_i32(&mut self, n: i32) {
        self.0 = self.0 << 32 | n as u64
    }
    fn write_i64(&mut self, n: i64) {
        self.0 = n as u64
    }
    fn write_isize(&mut self, n: isize) {
        self.0 = n as u64
    }

    fn finish(&self) -> u64 {
        self.0
    }
}
