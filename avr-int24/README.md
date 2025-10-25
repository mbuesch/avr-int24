# 24 bit integer arithmetic for AVR

This library provides a 24-bit signed integer type, `Int24`, for Rust.
It is designed for use on AVR microcontrollers.

## Features

- 24-bit signed integer type (`Int24`)
- Saturating arithmetic operations: addition, subtraction, multiplication, division
- Bitwise operations: shift left, shift right
- Specialized operations: Shift left and then divide
- Comparison operations
- Conversions to and from `i16` and `i32`
- Most operations are `const` or have a `const` variant
- All operations are highly optimized for speed and code size

## Usage

To use the `Int24` type, add `avr-int24` as a ependency to your `Cargo.toml`

```toml
[dependencies]
avr-int24 = "1"
```

and then use it in your code:

```rust
use avr_int24::Int24;

let a = Int24::from_i16(30000);
let b = Int24::from_i16(10000);

let c = a + b;

assert_eq!(c.to_i32(), 40000);
```

## Rust compiler

AVR inline assembly is not stabilized, yet.
Therefore, a `nightly` Rust compiler is required.
