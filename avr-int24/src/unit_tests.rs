// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use crate::Int24;

pub trait TestOps {
    fn print(&self, text: &str);
    fn print_num(&self, value: u32);
    fn begin(&self, name: &str);
    fn assert(&self, line: u16, ok: bool);
}

macro_rules! test_assert {
    ($test:expr, $ok:expr) => {
        $test.assert(core::line!() as _, $ok);
    };
}

fn test_conv_i16(t: &impl TestOps) {
    t.begin("conv_i16");

    let a = 0x1234;
    let b = Int24::from_i16(a).to_i16();
    test_assert!(t, a == b);

    let a = -0x1234;
    let b = Int24::from_i16(a).to_i16();
    test_assert!(t, a == b);

    let a = 0x123456;
    let b = Int24::from_i32(a).to_i16();
    test_assert!(t, b as u16 == 0x7FFF);

    let a = -0x123456;
    let b = Int24::from_i32(a).to_i16();
    test_assert!(t, b == -0x8000);
    test_assert!(t, b as u16 == 0x8000);

    let mut a = 0x0000_8000_u32;
    loop {
        let b = Int24::from_i32(a as i32).to_i16();
        test_assert!(t, b as u16 == 0x7FFF);
        if a == 0x4000_0000_u32 {
            break;
        }
        a <<= 1;
    }

    let mut a = 0xFFFF_8000_u32;
    loop {
        let b = Int24::from_i32(a as i32).to_i16();
        test_assert!(t, b as u16 == 0x8000);
        if a == 0x8000_0000_u32 {
            break;
        }
        a <<= 1;
    }
}

fn test_conv_i32(t: &impl TestOps) {
    t.begin("conv_i32");

    let a = 0x123456;
    let b = Int24::from_i32(a).to_i32();
    test_assert!(t, a == b);

    let a = -0x123456;
    let b = Int24::from_i32(a).to_i32();
    test_assert!(t, a == b);

    let a = 0x12345678;
    let b = Int24::from_i32(a).to_i32();
    test_assert!(t, b as u32 == 0x007F_FFFF);

    let a = -0x12345678;
    let b = Int24::from_i32(a).to_i32();
    test_assert!(t, b == -0x800000);
    test_assert!(t, b as u32 == 0xFF80_0000);

    let mut a = 0x0080_0000_u32;
    loop {
        let b = Int24::from_i32(a as i32).to_i32();
        test_assert!(t, b as u32 == 0x007F_FFFF);
        if a == 0x4000_0000_u32 {
            break;
        }
        a <<= 1;
    }

    let mut a = 0xFF80_0000_u32;
    loop {
        let b = Int24::from_i32(a as i32).to_i32();
        test_assert!(t, b as u32 == 0xFF80_0000);
        if a == 0x8000_0000_u32 {
            break;
        }
        a <<= 1;
    }
}

fn test_add(t: &impl TestOps) {
    t.begin("add");

    let a = Int24::from_i32(1000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(2010);
    test_assert!(t, a + b == c);
    test_assert!(t, a.const_add(b) == c);

    let a = Int24::from_i32(1000);
    let b = Int24::from_i32(-1010);
    let c = Int24::from_i32(-10);
    test_assert!(t, a + b == c);
    test_assert!(t, a.const_add(b) == c);

    let a = Int24::from_i32(-1000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(10);
    test_assert!(t, a + b == c);
    test_assert!(t, a.const_add(b) == c);

    let a = Int24::from_i32(0x7F_FFFF - 1);
    let b = Int24::from_i32(2);
    let c = Int24::from_i32(0x7F_FFFF);
    test_assert!(t, a + b == c);
    test_assert!(t, a.const_add(b) == c);

    let a = Int24::from_i32(-0x80_0000 + 1);
    let b = Int24::from_i32(-2);
    let c = Int24::from_i32(-0x80_0000);
    test_assert!(t, a + b == c);
    test_assert!(t, a.const_add(b) == c);
}

fn test_sub(t: &impl TestOps) {
    t.begin("sub");

    let a = Int24::from_i32(1000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(-10);
    test_assert!(t, a - b == c);
    test_assert!(t, a.const_sub(b) == c);

    let a = Int24::from_i32(1000);
    let b = Int24::from_i32(-1010);
    let c = Int24::from_i32(2010);
    test_assert!(t, a - b == c);
    test_assert!(t, a.const_sub(b) == c);

    let a = Int24::from_i32(-1000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(-2010);
    test_assert!(t, a - b == c);
    test_assert!(t, a.const_sub(b) == c);

    let a = Int24::from_i32(-0x80_0000 + 1);
    let b = Int24::from_i32(2);
    let c = Int24::from_i32(-0x80_0000);
    test_assert!(t, a - b == c);
    test_assert!(t, a.const_sub(b) == c);

    let a = Int24::from_i32(0x7F_FFFF - 1);
    let b = Int24::from_i32(-2);
    let c = Int24::from_i32(0x7F_FFFF);
    test_assert!(t, a - b == c);
    test_assert!(t, a.const_sub(b) == c);
}

fn test_mul(t: &impl TestOps) {
    t.begin("mul");

    let a = Int24::from_i32(1000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(1010000);
    test_assert!(t, a * b == c);
    test_assert!(t, a.const_mul(b) == c);

    let a = Int24::from_i32(1000);
    let b = Int24::from_i32(-1010);
    let c = Int24::from_i32(-1010000);
    test_assert!(t, a * b == c);
    test_assert!(t, a.const_mul(b) == c);

    let a = Int24::from_i32(-1000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(-1010000);
    test_assert!(t, a * b == c);
    test_assert!(t, a.const_mul(b) == c);

    let a = Int24::from_i32(0x7F_0000);
    let b = Int24::from_i32(2);
    let c = Int24::from_i32(0x7F_FFFF);
    test_assert!(t, a * b == c);
    test_assert!(t, a.const_mul(b) == c);

    let a = Int24::from_i32(-0x80_FFFF);
    let b = Int24::from_i32(2);
    let c = Int24::from_i32(-0x80_0000);
    test_assert!(t, a * b == c);
    test_assert!(t, a.const_mul(b) == c);
}

fn test_div(t: &impl TestOps) {
    t.begin("div");

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(99);
    test_assert!(t, a / b == c);
    test_assert!(t, a.const_div(b) == c);

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(-1010);
    let c = Int24::from_i32(-99);
    test_assert!(t, a / b == c);
    test_assert!(t, a.const_div(b) == c);

    let a = Int24::from_i32(-100000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(-99);
    test_assert!(t, a / b == c);
    test_assert!(t, a.const_div(b) == c);

    let a = Int24::from_i32(-0x80_0000);
    let b = Int24::from_i32(-1);
    let c = Int24::from_i32(0x7F_FFFF); // sat
    test_assert!(t, a / b == c);
    test_assert!(t, a.const_div(b) == c);
}

fn test_shl8div(t: &impl TestOps) {
    t.begin("shl8div");

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(25346);
    test_assert!(t, a.shl8div(b) == c);

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(-1010);
    let c = Int24::from_i32(-25346);
    test_assert!(t, a.shl8div(b) == c);

    let a = Int24::from_i32(-100000);
    let b = Int24::from_i32(1010);
    let c = Int24::from_i32(-25346);
    test_assert!(t, a.shl8div(b) == c);

    let a = Int24::from_i32(1000000);
    let b = Int24::from_i32(2);
    let c = Int24::from_i32(0x7FFFFF);
    test_assert!(t, a.shl8div(b) == c);
}

fn test_neg(t: &impl TestOps) {
    t.begin("neg");

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(-100000);
    test_assert!(t, -a == b);
    test_assert!(t, a.const_neg() == b);

    let a = Int24::from_i32(-100000);
    let b = Int24::from_i32(100000);
    test_assert!(t, -a == b);
    test_assert!(t, a.const_neg() == b);

    let a = Int24::from_i32(0x7F_FFFF);
    let b = Int24::from_i32(-0x7F_FFFF);
    test_assert!(t, -a == b);
    test_assert!(t, a.const_neg() == b);

    let a = Int24::from_i32(-0x7F_FFFF);
    let b = Int24::from_i32(0x7F_FFFF);
    test_assert!(t, -a == b);
    test_assert!(t, a.const_neg() == b);

    let a = Int24::from_i32(-0x80_0000);
    let b = Int24::from_i32(0x7F_FFFF); // saturated
    test_assert!(t, -a == b);
    test_assert!(t, a.const_neg() == b);
}

fn test_abs(t: &impl TestOps) {
    t.begin("abs");

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(100000);
    test_assert!(t, a.abs() == b);
    test_assert!(t, a.const_abs() == b);

    let a = Int24::from_i32(-100000);
    let b = Int24::from_i32(100000);
    test_assert!(t, a.abs() == b);
    test_assert!(t, a.const_abs() == b);

    let a = Int24::from_i32(0x7F_FFFF);
    let b = Int24::from_i32(0x7F_FFFF);
    test_assert!(t, a.abs() == b);
    test_assert!(t, a.const_abs() == b);

    let a = Int24::from_i32(-0x7F_FFFF);
    let b = Int24::from_i32(0x7F_FFFF);
    test_assert!(t, a.abs() == b);
    test_assert!(t, a.const_abs() == b);

    let a = Int24::from_i32(-0x80_0000);
    let b = Int24::from_i32(0x7F_FFFF); // saturated
    test_assert!(t, a.abs() == b);
    test_assert!(t, a.const_abs() == b);
}

fn test_shl(t: &impl TestOps) {
    t.begin("shl");

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(400000);
    test_assert!(t, a << 2 == b);
    test_assert!(t, a.const_shl(2) == b);

    let a = Int24::from_i32(1000);
    let b = Int24::from_i32(256000);
    test_assert!(t, a.shl8() == b);
}

fn test_shr(t: &impl TestOps) {
    t.begin("shr");

    let a = Int24::from_i32(400000);
    let b = Int24::from_i32(100000);
    test_assert!(t, a >> 2 == b);
    test_assert!(t, a.const_shr(2) == b);

    let a = Int24::from_i32(256000);
    let b = Int24::from_i32(1000);
    test_assert!(t, a.shr8() == b);
}

fn test_cmp(t: &impl TestOps) {
    t.begin("cmp");

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(100000);
    test_assert!(t, a == b);
    test_assert!(t, a.const_cmp(b) == core::cmp::Ordering::Equal);

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(100001);
    test_assert!(t, a != b);
    test_assert!(t, a.const_cmp(b) == core::cmp::Ordering::Less);

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(100000);
    test_assert!(t, a <= b);
    test_assert!(t, a.const_cmp(b) == core::cmp::Ordering::Equal);

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(100001);
    test_assert!(t, a < b);
    test_assert!(t, a.const_cmp(b) == core::cmp::Ordering::Less);

    let a = Int24::from_i32(100000);
    let b = Int24::from_i32(100000);
    test_assert!(t, a >= b);
    test_assert!(t, a.const_cmp(b) == core::cmp::Ordering::Equal);

    let a = Int24::from_i32(100001);
    let b = Int24::from_i32(100000);
    test_assert!(t, a > b);
    test_assert!(t, a.const_cmp(b) == core::cmp::Ordering::Greater);
}

pub fn run_tests(t: &impl TestOps) {
    t.print("\n\nBegin tests\n");
    test_conv_i16(t);
    test_conv_i32(t);
    test_add(t);
    test_sub(t);
    test_mul(t);
    test_div(t);
    test_shl8div(t);
    test_neg(t);
    test_abs(t);
    test_shl(t);
    test_shr(t);
    test_cmp(t);
    t.print("Done!\n");
}

// vim: ts=4 sw=4 expandtab
