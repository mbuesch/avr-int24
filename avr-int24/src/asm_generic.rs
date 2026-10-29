// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use crate::raw::IntS24Raw;

fn to_i32(a: IntS24Raw) -> i32 {
    if a.2 & 0x80 == 0 {
        i32::from_le_bytes([a.0, a.1, a.2, 0x00])
    } else {
        i32::from_le_bytes([a.0, a.1, a.2, 0xFF])
    }
}

fn from_i32(a: i32) -> IntS24Raw {
    let a = a.to_le_bytes();
    (a[0], a[1], a[2])
}

pub fn asm_mulsat24(a: IntS24Raw, b: IntS24Raw) -> IntS24Raw {
    let c = to_i32(a) as i64 * to_i32(b) as i64;
    if c > 0x7F_FFFF {
        from_i32(0x7F_FFFF)
    } else if c < -0x80_0000 {
        from_i32(-0x80_0000)
    } else {
        from_i32(c as i32)
    }
}

pub fn asm_divsat24(a: IntS24Raw, b: IntS24Raw, a_shl8: bool) -> IntS24Raw {
    if b == (0, 0, 0) {
        if a.2 & 0x80 == 0 {
            (0xFF, 0xFF, 0x7F)
        } else {
            (0x00, 0x00, 0x80)
        }
    } else if a == (0x00, 0x00, 0x80) && b == (0xFF, 0xFF, 0xFF) {
        (0xFF, 0xFF, 0x7F)
    } else {
        let mut a = to_i32(a);
        let b = to_i32(b);
        if a_shl8 {
            a <<= 8;
        }
        let c = a / b;
        if c > 0x7F_FFFF {
            from_i32(0x7F_FFFF)
        } else if c < -0x80_0000 {
            from_i32(-0x80_0000)
        } else {
            from_i32(c)
        }
    }
}

pub fn asm_negsat24(a: IntS24Raw) -> IntS24Raw {
    let b = from_i32(to_i32(a).wrapping_neg());
    if a.2 & 0x80 != 0 && b.2 & 0x80 != 0 {
        (0xFF, 0xFF, 0x7F)
    } else {
        b
    }
}

pub fn asm_shl24(a: IntS24Raw, count: u8) -> IntS24Raw {
    if count >= 24 {
        return from_i32(0);
    }
    from_i32(to_i32(a) << count)
}

pub fn asm_shr24(a: IntS24Raw, count: u8) -> IntS24Raw {
    from_i32(to_i32(a) >> count.min(23))
}

pub fn asm_ge24(a: IntS24Raw, b: IntS24Raw) -> bool {
    to_i32(a) >= to_i32(b)
}

// vim: ts=4 sw=4 expandtab
