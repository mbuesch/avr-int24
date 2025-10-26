// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

//! # 24 bit integer arithmetic for AVR
//!
//! This library provides a 24-bit signed integer type, `Int24`, for Rust.
//! It is designed for use on AVR microcontrollers.
//!
//! No operation from this crate ever panics.
//!
//! The operations don't overflow or underflow.
//! Numeric limits are handled by saturating the result instead.
//! One exception being the left shift operations, which don't saturate.
//!
//! Here are some example uses:
//!
//! ```
//! use avr_int24::Int24;
//!
//! let a = Int24::from_i16(30_000);
//! let b = Int24::from_i16(10_000);
//!
//! // Addition
//! let c = a + b;
//! assert_eq!(c.to_i32(), 40_000);
//!
//! // Subtraction
//! let c = b - a;
//! assert_eq!(c.to_i32(), -20_000);
//!
//! // Multiplication
//! let c = a * Int24::from_i16(-10);
//! assert_eq!(c.to_i32(), -300_000);
//!
//! // Division
//! let c = a / b;
//! assert_eq!(c.to_i32(), 3);
//!
//! // Negation
//! let c = -a;
//! assert_eq!(c.to_i32(), -30_000);
//!
//! // Arithmetic right shift
//! let c = a >> 2;
//! assert_eq!(c.to_i32(), 7_500);
//!
//! // Left shift
//! let c = a << 2;
//! assert_eq!(c.to_i32(), 120_000);
//!
//! // Saturation
//! let c = a * b;
//! assert_eq!(c.to_i32(), 0x7F_FFFF);
//! let c = a * -b;
//! assert_eq!(c.to_i32(), -0x80_0000);
//! ```

#![cfg_attr(not(test), no_std)]
#![cfg_attr(target_arch = "avr", feature(asm_experimental_arch))]

pub use crate::raw::Int24Raw;
use crate::raw::{
    abs24, add24,
    conv::{i16_to_i24raw, i24raw_to_i16_sat, i24raw_to_i32, i32_to_i24raw_sat},
    div24, eq24, ge24, mul24, neg24, raw_zero, shl24, shl24_by8, shl24_by8_div24, shl24_by16,
    shr24, shr24_by8, shr24_by16, sub24,
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

/// 24 bit signed integer.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Int24(Int24Raw);

#[allow(clippy::should_implement_trait)]
impl Int24 {
    /// Construct a new zero [Int24].
    pub const fn zero() -> Self {
        Self(raw_zero())
    }

    /// Construct a new zero [Int24].
    pub const fn new() -> Self {
        Self::zero()
    }

    /// Construct a new [Int24] from a little endian raw tuple.
    pub const fn from_raw(v: Int24Raw) -> Self {
        Self(v)
    }

    /// Construct a new [Int24] from raw little endian bytes.
    pub const fn from_le_bytes(bytes: [u8; 3]) -> Self {
        Self::from_raw((bytes[0], bytes[1], bytes[2]))
    }

    /// Construct a new [Int24] from a signed 16 bit integer.
    pub const fn from_i16(v: i16) -> Self {
        Self::from_raw(i16_to_i24raw(v))
    }

    /// Construct and saturate a new [Int24] from a signed 32 bit integer.
    pub const fn from_i32(v: i32) -> Self {
        Self(i32_to_i24raw_sat(v))
    }

    /// Convert this [Int24] to little endian bytes.
    pub const fn to_le_bytes(self) -> [u8; 3] {
        [self.0.0, self.0.1, self.0.2]
    }

    /// Convert and saturate this [Int24] to a signed 16 bit integer.
    pub const fn to_i16(self) -> i16 {
        i24raw_to_i16_sat(self.0)
    }

    /// Convert this [Int24] to a signed 32 bit integer.
    pub const fn to_i32(self) -> i32 {
        i24raw_to_i32(self.0)
    }

    /// Add and saturate two [Int24].
    #[inline(never)]
    pub fn add(self, other: Self) -> Self {
        Self::from_raw(add24(self.0, other.0))
    }

    /// Add and saturate two [Int24].
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::add] instead to get optimized code.
    pub const fn const_add(self, other: Self) -> Self {
        Self::from_i32(self.to_i32() + other.to_i32())
    }

    /// Subtract and saturate two [Int24].
    #[inline(never)]
    pub fn sub(self, other: Self) -> Self {
        Self::from_raw(sub24(self.0, other.0))
    }

    /// Subtract and saturate two [Int24].
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::sub] instead to get optimized code.
    pub const fn const_sub(self, other: Self) -> Self {
        Self::from_i32(self.to_i32() - other.to_i32())
    }

    /// Multiply and saturate two [Int24].
    #[inline(never)]
    pub fn mul(self, other: Self) -> Self {
        Self::from_raw(mul24(self.0, other.0))
    }

    /// Multiply and saturate two [Int24].
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::mul] instead to get optimized code.
    pub const fn const_mul(self, other: Self) -> Self {
        Self::from_i32(self.to_i32() * other.to_i32())
    }

    /// Divide and saturate two [Int24].
    #[inline(never)]
    pub fn div(self, other: Self) -> Self {
        Self::from_raw(div24(self.0, other.0))
    }

    /// Divide and saturate two [Int24].
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::div] instead to get optimized code.
    pub const fn const_div(self, other: Self) -> Self {
        Self::from_i32(self.to_i32() / other.to_i32())
    }

    /// Left shift `self` by 8 bits and then divide the shifted value by `other`.
    /// The result is saturated to signed 24 bit.
    /// The intermediate left shift by 8 bits is *not* saturated.
    ///
    /// The shifted intermediate value is kept in a big internal temporary memory
    /// which is not saturated.
    #[inline(never)]
    pub fn shl8div(self, other: Self) -> Self {
        Self::from_raw(shl24_by8_div24(self.0, other.0))
    }

    /// Left shift `self` by 8 bits and then divide the shifted value by `other`.
    /// The result is saturated to signed 24 bit.
    /// The intermediate left shift by 8 bits is *not* saturated.
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::shl8div] instead to get optimized code.
    pub fn const_shl8div(self, other: Self) -> Self {
        Self::from_i32((self.to_i32() << 8) / other.to_i32())
    }

    /// Two's complement negate and saturate `self`.
    #[inline(never)]
    pub fn neg(self) -> Self {
        Self(neg24(self.0))
    }

    /// Two's complement negate and saturate `self`.
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::neg] instead to get optimized code.
    pub const fn const_neg(self) -> Self {
        Self::from_i32(-self.to_i32())
    }

    /// Get the saturated absolute value of `self`.
    #[inline(never)]
    pub fn abs(self) -> Self {
        Self(abs24(self.0))
    }

    /// Get the saturated absolute value of `self`.
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::abs] instead to get optimized code.
    pub const fn const_abs(self) -> Self {
        if self.to_i32() < 0 {
            self.const_neg()
        } else {
            self
        }
    }

    /// Left shift `self` by 8 bits.
    ///
    /// This operation does not saturate the result.
    ///
    /// This operation is equivalent to calling `shl(8)`, but it is much faster.
    pub const fn shl8(self) -> Self {
        Self(shl24_by8(self.0))
    }

    /// Left shift `self` by 16 bits.
    ///
    /// This operation does not saturate the result.
    ///
    /// This operation is equivalent to calling `shl(16)`, but it is much faster.
    pub const fn shl16(self) -> Self {
        Self(shl24_by16(self.0))
    }

    /// Left shift `self` by `count` number of bits.
    ///
    /// This operation does not saturate the result.
    #[inline(never)]
    pub fn shl(self, count: u8) -> Self {
        Self(shl24(self.0, count))
    }

    /// Left shift `self` by `count` number of bits.
    /// This operation does not saturate the result.
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::shl] instead to get optimized code.
    pub const fn const_shl(self, count: u8) -> Self {
        Self::from_i32(self.to_i32() << count)
    }

    /// Arithmetically right shift `self` by 8 bits.
    ///
    /// This operation is equivalent to calling `shr(8)`, but it is much faster.
    pub const fn shr8(self) -> Self {
        Self(shr24_by8(self.0))
    }

    /// Arithmetically right shift `self` by 16 bits.
    ///
    /// This operation is equivalent to calling `shr(16)`, but it is much faster.
    pub const fn shr16(self) -> Self {
        Self(shr24_by16(self.0))
    }

    /// Arithmetically right shift `self` by `count` number of bits.
    #[inline(never)]
    pub fn shr(self, count: u8) -> Self {
        Self(shr24(self.0, count))
    }

    /// Arithmetically right shift `self` by `count` number of bits.
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::shr] instead to get optimized code.
    pub const fn const_shr(self, count: u8) -> Self {
        Self::from_i32(self.to_i32() >> count)
    }

    /// Compare `self` to `other` and return the result as [core::cmp::Ordering].
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

    /// Compare `self` to `other` and return the result as [core::cmp::Ordering].
    /// This is the `const` variant.
    ///
    /// Only call this from `const` context.
    /// From non-`const` context call [Int24::cmp] instead to get optimized code.
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

        fn print_num(&self, value: u32) {
            print!("{value}");
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
