// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

#![cfg_attr(not(test), no_std)]
#![cfg_attr(target_arch = "avr", feature(asm_experimental_arch))]

pub use crate::raw::Int24Raw;
use crate::raw::{
    abs24, add24,
    conv::{i16_to_i24raw, i24raw_to_i16_sat, i24raw_to_i32, i32_to_i24raw_sat},
    div24, eq24, ge24, mul24, neg24, raw_zero, shl24, shl24_by8, shr24, shr24_by8, sub24,
};

#[cfg(not(target_arch = "avr"))]
mod asm_generic;
#[cfg(not(target_arch = "avr"))]
use asm_generic as asm;

#[cfg(target_arch = "avr")]
mod asm_avr;
#[cfg(target_arch = "avr")]
use asm_avr as asm;

#[cfg(any(feature = "__internal_test__", test))]
pub mod unit_tests;

mod raw;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Int24(Int24Raw);

#[allow(clippy::should_implement_trait)]
impl Int24 {
    pub const fn zero() -> Self {
        Self(raw_zero())
    }

    pub const fn new() -> Self {
        Self::zero()
    }

    pub const fn from_raw(v: Int24Raw) -> Self {
        Self(v)
    }

    pub const fn from_i16(v: i16) -> Self {
        Self::from_raw(i16_to_i24raw(v))
    }

    pub const fn from_i32(v: i32) -> Self {
        Self(i32_to_i24raw_sat(v))
    }

    pub const fn to_i16(self) -> i16 {
        i24raw_to_i16_sat(self.0)
    }

    pub const fn to_i32(self) -> i32 {
        i24raw_to_i32(self.0)
    }

    #[inline(never)]
    pub fn add(self, other: Self) -> Self {
        Self::from_raw(add24(self.0, other.0))
    }

    pub const fn const_add(self, other: Self) -> Self {
        Self::from_i32(self.to_i32() + other.to_i32())
    }

    #[inline(never)]
    pub fn sub(self, other: Self) -> Self {
        Self::from_raw(sub24(self.0, other.0))
    }

    pub const fn const_sub(self, other: Self) -> Self {
        Self::from_i32(self.to_i32() - other.to_i32())
    }

    #[inline(never)]
    pub fn mul(self, other: Self) -> Self {
        Self::from_raw(mul24(self.0, other.0))
    }

    pub const fn const_mul(self, other: Self) -> Self {
        Self::from_i32(self.to_i32() * other.to_i32())
    }

    #[inline(never)]
    pub fn div(self, other: Self) -> Self {
        Self::from_raw(div24(self.0, other.0))
    }

    pub const fn const_div(self, other: Self) -> Self {
        Self::from_i32(self.to_i32() / other.to_i32())
    }

    #[inline(never)]
    pub fn neg(self) -> Self {
        Self(neg24(self.0))
    }

    pub const fn const_neg(self) -> Self {
        Self::from_i32(-self.to_i32())
    }

    #[inline(never)]
    pub fn abs(self) -> Self {
        Self(abs24(self.0))
    }

    pub const fn const_abs(self) -> Self {
        if self.to_i32() < 0 {
            self.const_neg()
        } else {
            self
        }
    }

    pub const fn shl8(self) -> Self {
        Self(shl24_by8(self.0))
    }

    #[inline(never)]
    pub fn shl(self, count: u8) -> Self {
        Self(shl24(self.0, count))
    }

    pub const fn const_shl(self, count: u8) -> Self {
        Self::from_i32(self.to_i32() << count)
    }

    pub const fn shr8(self) -> Self {
        Self(shr24_by8(self.0))
    }

    #[inline(never)]
    pub fn shr(self, count: u8) -> Self {
        Self(shr24(self.0, count))
    }

    pub const fn const_shr(self, count: u8) -> Self {
        Self::from_i32(self.to_i32() >> count)
    }

    #[inline(never)]
    pub fn cmp(self, other: Self) -> core::cmp::Ordering {
        if eq24(self.0, other.0) {
            core::cmp::Ordering::Equal
        } else if ge24(self.0, other.0) {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Less
        }
    }

    pub const fn const_cmp(self, other: Self) -> core::cmp::Ordering {
        if self.to_i32() == other.to_i32() {
            core::cmp::Ordering::Equal
        } else if self.to_i32() >= other.to_i32() {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Less
        }
    }
}

impl Default for Int24 {
    fn default() -> Self {
        Self::new()
    }
}

impl core::cmp::Ord for Int24 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Self::cmp(*self, *other)
    }
}

impl core::cmp::PartialOrd for Int24 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::ops::Add for Int24 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::add(self, other)
    }
}

impl core::ops::AddAssign for Int24 {
    fn add_assign(&mut self, other: Self) {
        self.0 = (*self + other).0;
    }
}

impl core::ops::Sub for Int24 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::sub(self, other)
    }
}

impl core::ops::SubAssign for Int24 {
    fn sub_assign(&mut self, other: Self) {
        self.0 = (*self - other).0;
    }
}

impl core::ops::Mul for Int24 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::mul(self, other)
    }
}

impl core::ops::MulAssign for Int24 {
    fn mul_assign(&mut self, other: Self) {
        self.0 = (*self * other).0;
    }
}

impl core::ops::Div for Int24 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::div(self, other)
    }
}

impl core::ops::DivAssign for Int24 {
    fn div_assign(&mut self, other: Self) {
        self.0 = (*self / other).0;
    }
}

impl core::ops::Neg for Int24 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl core::ops::Shl<u8> for Int24 {
    type Output = Self;

    fn shl(self, other: u8) -> Self {
        Self::shl(self, other)
    }
}

impl core::ops::ShlAssign<u8> for Int24 {
    fn shl_assign(&mut self, other: u8) {
        self.0 = (*self << other).0;
    }
}

impl core::ops::Shr<u8> for Int24 {
    type Output = Self;

    fn shr(self, other: u8) -> Self {
        Self::shr(self, other)
    }
}

impl core::ops::ShrAssign<u8> for Int24 {
    fn shr_assign(&mut self, other: u8) {
        self.0 = (*self >> other).0;
    }
}

#[cfg(test)]
mod test {
    use crate::unit_tests;

    struct TestRunner {}

    impl unit_tests::TestOps for TestRunner {
        fn print(&self, text: &str) {
            print!("{text}");
        }

        fn begin(&self, name: &str) {
            println!("Begin: {name}");
        }

        fn assert(&self, line: u16, ok: bool) {
            if ok {
                println!("line {line}: Ok");
            } else {
                panic!("line {line}: FAILED");
            }
        }
    }

    #[test]
    fn test_int24() {
        let t = TestRunner {};
        unit_tests::run_tests(&t);
    }
}

// vim: ts=4 sw=4 expandtab
