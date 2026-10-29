# 24 bit integer arithmetic for AVR

This library provides a 24-bit signed integer type, `IntS24` (alias `is24`), for Rust.
It is designed for use on AVR microcontrollers.

## Features

- 24-bit signed integer type (`IntS24` / `is24`)
- Saturating arithmetic operations: addition, subtraction, multiplication, division
- Bitwise operations: shift left, shift right
- Specialized operations: Shift left and then divide
- Comparison operations
- Conversions to and from `i8`, `i16` and `i32`
- Most operations are `const` or have a `const` variant
- All operations are highly optimized for speed and code size

## Usage

To use the `IntS24` / `is24` type, add `avr-int24` as a dependency to your `Cargo.toml`

```toml
[dependencies]
avr-int24 = "2"
```

and then use it in your code:

```rust
use avr_int24::is24;

let a = is24::from_i16(30000);
let b = is24::from_i16(10000);

let c = a + b;

assert_eq!(c.to_i32(), 40000);
```

## Rust compiler

AVR inline assembly is not yet stabilized.
Therefore, a `nightly` Rust compiler is required.
