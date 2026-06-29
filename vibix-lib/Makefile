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
BIN_DIR = target/$(TARGET)/release

.PHONY: all elf clean size symbols disasm

all: $(NAME).bin

elf:
	cargo build --release

$(NAME).bin: elf
	llvm-objcopy -O binary $(BIN_DIR)/$(NAME) $@
	@echo "Built $@ ($(shell du -h $@ | cut -f1))"

clean:
	cargo clean
	rm -f $(NAME).bin

size: $(NAME).bin
	@echo "Flat binary size: $$(stat --format=%s $(NAME).bin) bytes"
	@llvm-size $(BIN_DIR)/$(NAME)

symbols:
	nm -C $(BIN_DIR)/$(NAME) | sort | grep -E ' [Tt] '

disasm:
	llvm-objdump -d $(BIN_DIR)/$(NAME)
