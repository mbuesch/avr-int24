#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# SPDX-License-Identifier: Apache-2.0 OR MIT
# Copyright (C) 2025 Michael Büsch <m@bues.ch>

import sys, serial

s = serial.Serial(
    port=sys.argv[1],
    baudrate=19200,
)

buf = bytearray()
while True:
    b = s.read(1)
    if b == b'\n':
        print(buf.decode("UTF-8"))
        buf.clear()
    else:
        buf += b

# vim: ts=4 sw=4 expandtab
