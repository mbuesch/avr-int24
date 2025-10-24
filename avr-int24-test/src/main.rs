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
    #[inline(never)]
    fn print(&self, text: &str) {
        self.uart.tx_str(text);
    }

    #[inline(never)]
    fn print_num(&self, value: u32) {
        let mut buf = itoa::Buffer::new();
        self.print(buf.format(value));
    }

    #[inline(never)]
    fn begin(&self, name: &str) {
        self.print("Begin: ");
        self.print(name);
        self.print("\n");
    }

    #[inline(never)]
    fn assert(&self, line: u16, ok: bool) {
        self.print("line ");
        self.print_num(line.into());
        if ok {
            self.print(": Ok\n");
        } else {
            self.print(": FAILED\n");
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

    loop {
        avr_device::interrupt::disable();
        avr_device::asm::sleep();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        avr_device::interrupt::disable();
        avr_device::asm::sleep();
    }
}

// vim: ts=4 sw=4 expandtab
