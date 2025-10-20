// -*- coding: utf-8 -*-
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (C) 2025 Michael Büsch <m@bues.ch>

use crate::pac::USART0;

pub struct Uart {
    dp: USART0,
}

impl Uart {
    #[rustfmt::skip]
    pub fn new(dp: USART0) -> Self {
        dp.udr0.write(|w| {
            w.bits(0)
        });
        dp.ubrr0.write(|w| {
            w.bits(16) // 115200 baud
        });
        dp.ucsr0a.write(|w| {
            w.u2x0().set_bit()
        });
        dp.ucsr0c.write(|w| {
            w.umsel0().usart_async()
             .ucsz0().chr8()
             .usbs0().stop1()
             .upm0().disabled()
        });
        dp.ucsr0b.write(|w| {
            w.txen0().set_bit()
             .rxen0().clear_bit()
        });

        Self { dp }
    }

    pub fn tx_byte(&self, data: u8) {
        while !self.dp.ucsr0a.read().udre0().bit() {
            // wait for previous tx to finish.
        }
        self.dp.udr0.write(|w| w.bits(data));
    }

    pub fn tx_slice(&self, data: &[u8]) {
        for d in data {
            self.tx_byte(*d);
        }
    }

    pub fn tx_str(&self, s: &str) {
        self.tx_slice(s.as_bytes());
    }
}

// vim: ts=4 sw=4 expandtab
