# VIBIX flat binary build
#
# Targets:
#   all      — build the flat binary
#   elf      — build the ELF (debug)
#   clean    — remove build artifacts
#   size     — show binary size
#   symbols  — show demangled symbols
#   disasm   — disassemble the binary

TARGET = x86_64-unknown-none
NAME = vibix
CARGO ?= cargo
OBJCOPY ?= objcopy
CARGO_TARGET_DIR ?= target
BIN_DIR = $(CARGO_TARGET_DIR)/$(TARGET)/release

.PHONY: all elf clean size symbols disasm

all: $(NAME).bin

elf:
	CARGO_TARGET_DIR="$(CARGO_TARGET_DIR)" $(CARGO) build --release --offline

$(NAME).bin: elf
	$(OBJCOPY) -O binary $(BIN_DIR)/$(NAME) $@
	@echo "Built $@"
	@du -h $@

clean:
	CARGO_TARGET_DIR="$(CARGO_TARGET_DIR)" $(CARGO) clean
	rm -f $(NAME).bin

size: $(NAME).bin
	@echo "Flat binary size: $$(stat --format=%s $(NAME).bin) bytes"
	@size $(BIN_DIR)/$(NAME)

symbols:
	nm -C $(BIN_DIR)/$(NAME) | sort | grep -E ' [Tt] '

disasm:
	objdump -d $(BIN_DIR)/$(NAME)
