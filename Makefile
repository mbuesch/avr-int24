.PHONY: all clean dasm isp isp-fuses isp-flash dw dw-flash

all:
	$(MAKE) -C avr-int24 test
	$(MAKE) -C avr-int24-test all

clean:
	$(MAKE) -C avr-int24-test clean

dasm:
	$(MAKE) -C avr-int24-test dasm

isp:
	$(MAKE) -C avr-int24-test isp

isp-fuses:
	$(MAKE) -C avr-int24-test isp-fuses

isp-flash:
	$(MAKE) -C avr-int24 test
	$(MAKE) -C avr-int24-test isp-flash

dw:
	$(MAKE) -C avr-int24-test dw

dw-flash:
	$(MAKE) -C avr-int24 test
	$(MAKE) -C avr-int24-test dw-flash
