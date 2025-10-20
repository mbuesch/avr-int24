// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

mod uart;

use crate::uart::Uart;
use avr_device::atmega328p as pac;
use avr_int24::unit_tests::{TestOps, run_tests};

struct TestRunner<'a> {
    uart: &'a Uart,
}

impl<'a> TestOps for TestRunner<'a> {
    fn print(&self, text: &str) {
        self.uart.tx_str(text);
    }

    #[inline(never)]
    fn begin(&self, name: &str) {
        self.uart.tx_str("Begin: ");
        self.uart.tx_str(name);
        self.uart.tx_str("\n");
    }

    #[inline(never)]
    fn assert(&self, line: u16, ok: bool) {
        self.uart.tx_str("line ");
        let mut buf = itoa::Buffer::new();
        self.uart.tx_str(buf.format(line));
        if ok {
            self.uart.tx_str(": Ok\n");
        } else {
            self.uart.tx_str(": FAILED\n");
            panic!();
        }
    }
}

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let uart = Uart::new(dp.USART0);
    let test = TestRunner { uart: &uart };

    run_tests(&test);

    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        avr_device::interrupt::disable();
    }
}

// vim: ts=4 sw=4 expandtab
