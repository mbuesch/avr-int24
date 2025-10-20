OBJCOPY:=avr-objcopy
OBJDUMP:=avr-objdump
GDB:=avr-gdb
SIZE:=avr-size

DWDEBUG:=dwdebug
DWDEBUG_PORT:=ttyUSB0

AVRDUDE:=avrdude
AVRDUDE_MCU:=m328p
AVRDUDE_PORT:=usb
AVRDUDE_PROG:=avrisp2
AVRDUDE_SPEED:=1
AVRDUDE_SPEED_SLOW:=10

# Fuses: ext-clock=65ms; BOD=4.3V
LFUSE:=0xE0
HFUSE:=0xD9
EFUSE:=0xFC

AVR_CPU_FREQUENCY_HZ:=16000000

NAME:=avr-int24-test
TARGET:=avr-atmega328p
RELEASEDIR:=target/$(TARGET)/release
BIN:=$(RELEASEDIR)/$(NAME).bin
HEX:=$(RELEASEDIR)/$(NAME).hex
ELF:=$(RELEASEDIR)/$(NAME).elf
DASM:=$(RELEASEDIR)/$(NAME).dasm

all: $(BIN) $(DASM)
	@-echo
	@-$(SIZE) --format=SysV $(ELF) | grep -Ee '^(\.data|\.bss|\.text)'

$(BIN): $(HEX)
	$(OBJCOPY) -I ihex -O binary $(HEX) $(BIN)

$(HEX): $(ELF)
	$(OBJCOPY) -R.eeprom -O ihex $(ELF) $(HEX)

$(ELF):
	AVR_CPU_FREQUENCY_HZ=$(AVR_CPU_FREQUENCY_HZ) \
	cargo build --release

.PHONY: $(ELF) # Always run cargo

$(DASM): $(HEX)
	$(OBJDUMP) -j .text --disassemble --disassemble-zeroes $(ELF) > $(DASM)

dasm: $(DASM)
	less $(DASM)

clean:
	cargo clean

isp:
	$(AVRDUDE) -P $(AVRDUDE_PORT) -c $(AVRDUDE_PROG) \
		-B $(AVRDUDE_SPEED_SLOW) -p $(AVRDUDE_MCU) \
		-t

isp-fuses:
	$(AVRDUDE) -P $(AVRDUDE_PORT) -c $(AVRDUDE_PROG) \
		-B $(AVRDUDE_SPEED_SLOW) -p $(AVRDUDE_MCU) \
		-U lfuse:w:$(LFUSE):m -U hfuse:w:$(HFUSE):m -U efuse:w:$(EFUSE):m

isp-flash: $(HEX)
	$(AVRDUDE) -P $(AVRDUDE_PORT) -c $(AVRDUDE_PROG) \
		-B $(AVRDUDE_SPEED) -p $(AVRDUDE_MCU) \
		-U flash:w:$(HEX)

dw:
	$(DWDEBUG) 'device $(DWDEBUG_PORT), te'

dw-flash: $(BIN)
	$(DWDEBUG) 'device $(DWDEBUG_PORT), l $(BIN), te, qr'
